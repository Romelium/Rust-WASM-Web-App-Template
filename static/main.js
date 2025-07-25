// static/main.js
// @ts-check
import { initializeApp } from './js/appManager.js';

/**
 * Initializes and runs the application.
 * @returns {Promise<import('./pkg/app.js').AppHandle|null>}
 */
export async function run_app() {
    return await initializeApp();
}
