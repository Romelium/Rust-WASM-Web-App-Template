// e2e_tests/consoleUtils.js
// @ts-check

/**
 * @typedef {object} ConsoleMessage
 * @property {string} text
 * @property {string} type
 */

/**
 * Filters out known benign console errors.
 * @param {ConsoleMessage[]} allMessages
 * @returns {ConsoleMessage[]}
 */
export function getCriticalConsoleErrors(allMessages) {
  const errors = allMessages.filter(msg => msg.type === 'error');
  return errors.filter(err => {
    const text = err.text;
    if (text.includes('favicon.ico')) return false;
    // Add more patterns for benign errors if they appear
    return true;
  });
}
