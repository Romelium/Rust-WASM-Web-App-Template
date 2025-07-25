// static/js/appManager.js
// @ts-check
import init, { mount_app } from '../pkg/app.js';

/**
 * Initializes the WASM application, sets up rendering and input.
 * @returns {Promise<import('../pkg/app.js').AppHandle|null>}
 */
export async function initializeApp() {
    try {
        await init();

        // Mount the application. The Rust code will find the container element
        // by its hardcoded ID and create the canvas and button elements inside it.
        const appInstance = mount_app();

        // The renderer still needs to be initialized after the app is created
        // to get access to the canvas's 2D context.
        appInstance.initializeRenderer();
        
        // --- Start the Render Loop (now managed by Rust) ---
        appInstance.start();

        return appInstance;
    } catch (e) {
        console.error("Error initializing WASM App:", e);
        return null;
    }
}
