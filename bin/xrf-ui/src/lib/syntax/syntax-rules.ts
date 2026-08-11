import { ESyntaxLanguage, ESyntaxToken, ISyntaxRule } from "@/lib/syntax/syntax.types";

/**
 * X-Ray configuration.
 *
 * Not quite the ini every highlighter ships: a section header carries its inherited parents after a
 * colon (`[wpn_ak74]:wpn_base`), and `#include` pulls another file in. Both are the first thing worth
 * seeing in a config, and both are what a stock ini grammar gets wrong.
 */
const LTX_RULES: Array<ISyntaxRule> = [
  { token: ESyntaxToken.COMMENT, pattern: "(?:;|//)[^\\n]*" },
  { token: ESyntaxToken.DIRECTIVE, pattern: "^[ \\t]*#[A-Za-z_]+" },
  { token: ESyntaxToken.SECTION, pattern: "^[ \\t]*\\[[^\\]\\n]*\\]" },
  // Everything after `]:` on a section line names a parent section rather than a value.
  { token: ESyntaxToken.TYPE, pattern: "(?<=\\]:)[^;\\n]*" },
  { token: ESyntaxToken.STRING, pattern: '"[^"\\n]*"' },
  // A key is whatever precedes the `=` on its line, which is the only way to tell it from a value.
  { token: ESyntaxToken.KEY, pattern: "^[ \\t]*[A-Za-z_$][^=;\\n]*?(?=[ \\t]*=)" },
  { token: ESyntaxToken.NUMBER, pattern: "\\b\\d+(?:\\.\\d+)?\\b" },
  { token: ESyntaxToken.OPERATOR, pattern: "[=,:]" },
];

const LUA_KEYWORDS: string =
  "and|break|do|elseif|else|end|false|for|class|function|goto|if|in|local|nil|not|or|" +
  "repeat|return|then|true|until|while";

const LUA_BUILTINS: string =
  "self|_G|assert|error|ipairs|pairs|pcall|print|require|select|setmetatable|time_global|" +
  "getmetatable|tonumber|tostring|type|unpack|string|table|math|io|os|coroutine";

/**
 * Lua, covering both `.script` game scripts and the `.s` shader scripts, which are Lua as well.
 */
const LUA_RULES: Array<ISyntaxRule> = [
  // Long forms first: `--[[` also matches the line comment pattern, and would end at the first newline.
  { token: ESyntaxToken.COMMENT, pattern: "--\\[\\[[\\s\\S]*?\\]\\]|--\\[=\\[[\\s\\S]*?\\]=\\]|--[^\\n]*" },
  {
    token: ESyntaxToken.STRING,
    pattern: "\\[\\[[\\s\\S]*?\\]\\]|\\[=\\[[\\s\\S]*?\\]=\\]|\"(?:\\\\.|[^\"\\\\\\n])*\"|'(?:\\\\.|[^'\\\\\\n])*'",
  },
  { token: ESyntaxToken.NUMBER, pattern: "\\b0[xX][0-9a-fA-F]+\\b|\\b\\d+(?:\\.\\d+)?(?:[eE][+-]?\\d+)?\\b" },
  { token: ESyntaxToken.KEYWORD, pattern: `\\b(?:${LUA_KEYWORDS})\\b` },
  { token: ESyntaxToken.TYPE, pattern: `\\b(?:${LUA_BUILTINS})\\b` },
  { token: ESyntaxToken.OPERATOR, pattern: "\\.\\.\\.|\\.\\.|[=~<>]=|[+\\-*/%^#=<>(){}\\[\\];:,.]" },
];

const SHADER_KEYWORDS: string =
  "uniform|static|const|extern|shared|volatile|inline|typedef|namespace|return|if|else|for|while|do|" +
  "switch|case|default|break|continue|discard|in|out|inout|register|packoffset|cbuffer|tbuffer|" +
  "technique|pass|compile|struct|true|false";

const SHADER_TYPES: string =
  "(?:float|half|double|int|uint|min16float|min10float|bool)[1-4]?(?:x[1-4])?|" +
  "void|matrix|vector|string|sampler(?:1D|2D|3D|CUBE|_state)?|SamplerState|SamplerComparisonState|" +
  "Texture(?:1D|2D|3D|Cube)(?:Array)?|(?:RW)?(?:Structured|ByteAddress)?Buffer|InputPatch|OutputPatch|" +
  "PointStream|LineStream|TriangleStream";

/**
 * HLSL, as used by `.ps`, `.vs`, `.hs`, `.ds` and the `.h` headers they include.
 */
const SHADER_RULES: Array<ISyntaxRule> = [
  { token: ESyntaxToken.COMMENT, pattern: "/\\*[\\s\\S]*?\\*/|//[^\\n]*" },
  // The space is real: these files are full of `# ifndef` and `#  define`.
  { token: ESyntaxToken.DIRECTIVE, pattern: "^[ \\t]*#[ \\t]*[A-Za-z_]+" },
  { token: ESyntaxToken.STRING, pattern: '"[^"\\n]*"' },
  {
    token: ESyntaxToken.NUMBER,
    pattern: "\\b0[xX][0-9a-fA-F]+\\b|\\b\\d+\\.?\\d*(?:[eE][+-]?\\d+)?[fFhHlLuU]?\\b|\\.\\d+[fFhH]?\\b",
  },
  { token: ESyntaxToken.TYPE, pattern: `\\b(?:${SHADER_TYPES})\\b` },
  { token: ESyntaxToken.KEYWORD, pattern: `\\b(?:${SHADER_KEYWORDS})\\b` },
  { token: ESyntaxToken.OPERATOR, pattern: "[+\\-*/%=<>!&|^~?:;,.(){}\\[\\]]" },
];

/**
 * XML, covering configs, UI layouts and dialogs.
 *
 * Attributes are found by the `=` that follows them rather than by tracking whether the scan is inside
 * a tag, so `word=` in text content would colour as one. Game XML does not do that in practice, and the
 * cost of being wrong is a differently coloured word in a read-only view.
 */
const XML_RULES: Array<ISyntaxRule> = [
  { token: ESyntaxToken.COMMENT, pattern: "<!--[\\s\\S]*?-->" },
  { token: ESyntaxToken.DIRECTIVE, pattern: "<[!?][\\s\\S]*?>" },
  { token: ESyntaxToken.SECTION, pattern: "</?[A-Za-z_][\\w:.\\-]*" },
  { token: ESyntaxToken.STRING, pattern: "\"[^\"]*\"|'[^']*'" },
  { token: ESyntaxToken.KEY, pattern: "[A-Za-z_][\\w:.\\-]*(?=\\s*=)" },
  { token: ESyntaxToken.OPERATOR, pattern: "/?>|=" },
];

const SYNTAX_RULES: Record<ESyntaxLanguage, Array<ISyntaxRule>> = {
  [ESyntaxLanguage.PLAIN]: [],
  [ESyntaxLanguage.LTX]: LTX_RULES,
  [ESyntaxLanguage.LUA]: LUA_RULES,
  [ESyntaxLanguage.SHADER]: SHADER_RULES,
  [ESyntaxLanguage.XML]: XML_RULES,
};

/**
 * Rules colouring one language, in the order they take precedence at a given position.
 *
 * @param language - Language to describe.
 * @returns Its ordered rules, empty for a language with nothing to colour.
 */
export function getSyntaxRules(language: ESyntaxLanguage): Array<ISyntaxRule> {
  return SYNTAX_RULES[language] ?? [];
}
