// e2e_tests/testUtils.js
// @ts-check

/**
 * Calls a method on the window.appInstance.
 * @param {import('@playwright/test').Page} page
 * @param {string} methodName
 * @param {any[]} [args=[]]
 * @returns {Promise<any>}
 */
export async function callWasmMethod(page, methodName, args = []) {
  return await page.evaluate(
    // @ts-ignore
    async ({ method, callArgs }) => {
      if (!window.appInstance) {
        throw new Error("window.appInstance is not available for this test.");
      }
      // @ts-ignore
      return await window.appInstance[method](...callArgs)
    },
    { method: methodName, callArgs: args }
  );
}
