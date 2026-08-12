import { LTX_RULES } from "@/lib/syntax/syntax.ltx.rules";
import { LUA_RULES } from "@/lib/syntax/syntax.lua.rules";
import { SHADER_RULES } from "@/lib/syntax/syntax.shader.rules";
import { ESyntaxLanguage, ISyntaxRule } from "@/lib/syntax/syntax.types";
import { TYPESCRIPT_RULES } from "@/lib/syntax/syntax.typescript.rules";
import { XML_RULES } from "@/lib/syntax/syntax.xml.rules";

const SYNTAX_RULES: Record<ESyntaxLanguage, Array<ISyntaxRule>> = {
  [ESyntaxLanguage.PLAIN]: [],
  [ESyntaxLanguage.LTX]: LTX_RULES,
  [ESyntaxLanguage.LUA]: LUA_RULES,
  [ESyntaxLanguage.SHADER]: SHADER_RULES,
  [ESyntaxLanguage.TYPESCRIPT]: TYPESCRIPT_RULES,
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
