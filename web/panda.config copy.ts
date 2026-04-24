import { animationStyles } from "./src/theme/animationStyles";
import { amber } from "./src/theme/colors/amber";
import { green } from "./src/theme/colors/green";
import { lime } from "./src/theme/colors/lime";
import { red } from "./src/theme/colors/red";
import { sand } from "./src/theme/colors/sand";
import { conditions } from "./src/theme/conditions";
import { globalCss } from "./src/theme/globalCss";
import { keyframes } from "./src/theme/keyframes";
import { layerStyles } from "./src/theme/layerStyles";
import { recipes, slotRecipes } from "./src/theme/recipes";
import { textStyles } from "./src/theme/textStyles";
import { colors } from "./src/theme/tokens/colors";
import { durations } from "./src/theme/tokens/durations";
import { shadows } from "./src/theme/tokens/shadows";
import { zIndex } from "./src/theme/tokens/z-index";
import { defineConfig } from "@pandacss/dev";

export default defineConfig({
  // Whether to use css reset
  preflight: true,

  // Where to look for your css declarations
  include: ["./src/**/*.{js,jsx,ts,tsx}", "./pages/**/*.{js,jsx,ts,tsx}"],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {
      animationStyles: animationStyles,
      recipes: recipes,
      slotRecipes: slotRecipes,
      keyframes: keyframes,
      layerStyles: layerStyles,
      textStyles: textStyles,

      tokens: {
        colors: colors,
        durations: durations,
        zIndex: zIndex,
      },

      semanticTokens: {
        colors: {
          fg: {
            default: {
              value: {
                _light: "{colors.gray.12}",
                _dark: "{colors.gray.12}",
              },
            },

            muted: {
              value: {
                _light: "{colors.gray.11}",
                _dark: "{colors.gray.11}",
              },
            },

            subtle: {
              value: {
                _light: "{colors.gray.10}",
                _dark: "{colors.gray.10}",
              },
            },
          },

          border: {
            value: {
              _light: "{colors.gray.4}",
              _dark: "{colors.gray.4}",
            },
          },

          error: {
            value: {
              _light: "{colors.red.9}",
              _dark: "{colors.red.9}",
            },
          },

          lime: lime,
          gray: sand,
          red: red,
          green: green,
          amber: amber,
        },

        shadows: shadows,

        radii: {
          l1: {
            value: "{radii.xs}",
          },

          l2: {
            value: "{radii.sm}",
          },

          l3: {
            value: "{radii.md}",
          },
        },
      },
    },
  },

  // The output directory for your css system
  outdir: "styled-system",

  globalCss: globalCss,
  conditions: conditions,
});
