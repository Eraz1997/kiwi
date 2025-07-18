import { createBackendClient } from "./createBackendClient";
import argon2 from "argon2-browser/dist/argon2-bundled.min.js";
import forge from "node-forge";

const LOCAL_ENCRYPTION_KEY_LOCAL_STORAGE_ITEM =
  "_kiwi_sealed_local_encryption_key";
const ENCRYPTION_VALIDATION_SUFFIX = ":_kiwi_valid_decrypted_key";

type CredentialsClient = {
  getLoginPasswordHash: (password: string) => Promise<string>;
  storeAndSealLocalEncryptionKey: (
    username: string,
    password: string,
  ) => Promise<void>;
};

export const createCredentialsClient = (): CredentialsClient => {
  const authBackendClient = createBackendClient("auth");

  const getLoginPasswordHash = async (password: string) => {
    const hash = await argon2.hash({
      pass: password,
      hashLen: 64,
      time: 3,
      mem: 65_536,
      parallelism: 1,
      salt: "login-call",
      type: argon2.ArgonType.Argon2id,
    });
    return hash.hashHex;
  };

  const storeAndSealLocalEncryptionKey = async (
    username: string,
    password: string,
  ) => {
    const textEncoder = new TextEncoder();
    const localEncryptionKey = await argon2.hash({
      pass: password,
      hashLen: 64,
      time: 3,
      mem: 65_536,
      parallelism: 1,
      salt: "local-encryption-key",
      secret: textEncoder.encode(username),
      type: argon2.ArgonType.Argon2id,
    });

    const { jsonPayload } = await authBackendClient.get("/sealing-key");
    const { key, iv } = jsonPayload;
    const textToEncrypt = `${localEncryptionKey.hashHex}${ENCRYPTION_VALIDATION_SUFFIX}`;

    const cipher = forge.cipher.createCipher(
      "AES-CBC",
      forge.util.createBuffer(key),
    );
    cipher.start({ iv: forge.util.createBuffer(iv) });
    cipher.update(forge.util.createBuffer(textToEncrypt));
    cipher.finish();

    const sealedEncryptionKey = cipher.output.toHex();

    window.localStorage.setItem(
      LOCAL_ENCRYPTION_KEY_LOCAL_STORAGE_ITEM,
      sealedEncryptionKey,
    );
  };

  return {
    getLoginPasswordHash,
    storeAndSealLocalEncryptionKey,
  };
};
