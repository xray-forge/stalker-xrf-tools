export interface ITranslationJson {
  [language: string]: string | Array<string>;
}

export interface ITranslationsProjectJson {
  [file: string]: ITranslationJson;
}
