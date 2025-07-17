import { createBackendClient } from "./createBackendClient";
import argon2 from "argon2-browser/dist/argon2-bundled.min.js";

const LOCAL_ENCRYPTION_KEY_LOCAL_STORAGE_ITEM =
  "_kiwi_sealed_local_encryption_key";
const SHARED_IV = "_kiwi_shared_iv_";

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
    const { sealing_key: sealingKey } = jsonPayload;
    const sealedEncryptionKey = await window.crypto.subtle.encrypt(
      {
        name: "AES-CBC",
        iv: textEncoder.encode(SHARED_IV),
      },
      sealingKey,
      textEncoder.encode(localEncryptionKey.hashHex),
    );
    const hexSealedEncryptionKey = window.btoa(
      String.fromCharCode(...new Uint8Array(sealedEncryptionKey)),
    );

    window.localStorage.setItem(
      LOCAL_ENCRYPTION_KEY_LOCAL_STORAGE_ITEM,
      hexSealedEncryptionKey,
    );
  };

  return {
    getLoginPasswordHash,
    storeAndSealLocalEncryptionKey,
  };
};
