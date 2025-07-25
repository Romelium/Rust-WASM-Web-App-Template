/// <reference path="./global.d.ts" />
// @ts-check
const { test, expect } = require('@playwright/test');
const { getCriticalConsoleErrors } = require('./consoleUtils.js');

/** @type {Array<{text: string, type: string}>} */
let consoleMessages = [];

test.beforeEach(async ({ page }) => {
  consoleMessages = [];
  page.on('console', msg => {
    console.log(`BROWSER LOG: ${msg.text()}`);
    consoleMessages.push({ text: msg.text(), type: msg.type() });
  });

  await page.goto('/');
  await page.waitForFunction(() => window.appInstance !== undefined, null, { timeout: 10000 });
});

test.afterEach(async () => {
  const criticalErrors = getCriticalConsoleErrors(consoleMessages);
  if (criticalErrors.length > 0) {
    console.error("Unexpected console errors found:", JSON.stringify(criticalErrors, null, 2));
  }
  expect(criticalErrors).toEqual([]);
});

test('application initializes and canvas is visible', async ({ page }) => {
  await expect(page.locator('canvas#drawing-canvas')).toBeVisible();
  const initLog = consoleMessages.some(msg => msg.text.includes("WASM module loaded."));
  expect(initLog).toBe(true);
});

test('clicking canvas adds a shape, and clearing removes it', async ({ page }) => {
  const canvas = page.locator('canvas#drawing-canvas');
  const clearButton = page.locator('button#clear-btn');

  /**
   * A helper function to get the state from the WASM module.
   * This runs in the browser context.
   * @returns {{shapes: any[]}}
   */
  const getWasmState = () => {
    if (!window.appInstance) {
      throw new Error("window.appInstance is not available.");
    }
    return window.appInstance.getDrawingState();
  };

  // 1. Verify initial state is empty
  let state = await page.evaluate(getWasmState);
  expect(state.shapes.length).toBe(0);

  // 2. Click to add a shape
  await canvas.click({ position: { x: 100, y: 150 } });

  // 3. Verify a shape was added
  state = await page.evaluate(getWasmState);
  expect(state.shapes.length).toBe(1);
  
  // A tolerance is needed because sub-pixel layout can slightly scale click coordinates.
  // We assert the coordinate is within a reasonable range (e.g., +/- 1 pixel) of the target.
  expect(state.shapes[0].x).toBeGreaterThan(99);
  expect(state.shapes[0].x).toBeLessThan(101);
  expect(state.shapes[0].y).toBeGreaterThan(149);
  expect(state.shapes[0].y).toBeLessThan(151);

  // 4. Click to add another shape
  await canvas.click({ position: { x: 250, y: 200 } });
  state = await page.evaluate(getWasmState);
  expect(state.shapes.length).toBe(2);
  expect(state.shapes[1].x).toBeGreaterThan(249);
  expect(state.shapes[1].x).toBeLessThan(251);
  expect(state.shapes[1].y).toBeGreaterThan(199);
  expect(state.shapes[1].y).toBeLessThan(201);

  // 5. Click the clear button
  await clearButton.click();

  // 6. Verify the state is empty again
  state = await page.evaluate(getWasmState);
  expect(state.shapes.length).toBe(0);
});
