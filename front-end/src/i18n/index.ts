import { createI18n } from 'vue-i18n'

import en from '@/locales/en.json'
import fr from '@/locales/fr.json'
import ja from '@/locales/ja.json'

export const SUPPORTED_LOCALES = ['en', 'fr', 'ja'] as const
export type Locale = (typeof SUPPORTED_LOCALES)[number]

export const DEFAULT_LOCALE: Locale = 'en'

/** Pick a starting locale from a saved preference or the browser, falling back to the default. */
function resolveInitialLocale(): Locale {
  const stored = localStorage.getItem('locale')
  if (stored && (SUPPORTED_LOCALES as readonly string[]).includes(stored)) {
    return stored as Locale
  }

  const browser = navigator.language.split('-')[0]
  if (browser && (SUPPORTED_LOCALES as readonly string[]).includes(browser)) {
    return browser as Locale
  }

  return DEFAULT_LOCALE
}

const i18n = createI18n({
  legacy: false,
  locale: resolveInitialLocale(),
  fallbackLocale: DEFAULT_LOCALE,
  messages: { en, fr, ja },
})

export default i18n
