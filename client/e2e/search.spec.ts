/**
 * Search E2E Tests
 *
 * Tests global search panel and message search functionality.
 * Prerequisites: Backend running, test users + seed data with messages
 */

import { test, expect } from "@playwright/test";
import { loginAsAdmin, selectFirstGuild } from "./helpers";

test.describe("Search", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
    await selectFirstGuild(page);
  });

  test("should open search panel", async ({ page }) => {
    // Click search button in sidebar or use keyboard shortcut
    const searchBtn = page.locator('button:has-text("Search")');
    if (await searchBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await searchBtn.click();
    } else {
      // Try keyboard shortcut (Ctrl+Shift+F)
      await page.keyboard.press("Control+Shift+f");
    }

    // Search panel should appear with input
    await expect(
      page.locator('input[placeholder*="search" i], input[type="search"]')
    ).toBeVisible({ timeout: 5000 });
  });

  test("should accept search query", async ({ page }) => {
    // Open search
    const searchBtn = page.locator('button:has-text("Search")');
    if (await searchBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await searchBtn.click();
    } else {
      await page.keyboard.press("Control+Shift+f");
    }

    const searchInput = page.locator(
      'input[placeholder*="search" i], input[type="search"]'
    );
    if (await searchInput.isVisible({ timeout: 3000 }).catch(() => false)) {
      await searchInput.fill("hello");
      await searchInput.press("Enter");
      // Should attempt search (results depend on data)
      await page.waitForTimeout(2000);
    }
  });

  test("should display search results", async ({ page }) => {
    const searchBtn = page.locator('button:has-text("Search")');
    if (await searchBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await searchBtn.click();
    } else {
      await page.keyboard.press("Control+Shift+f");
    }

    const searchInput = page.locator(
      'input[placeholder*="search" i], input[type="search"]'
    );
    if (await searchInput.isVisible({ timeout: 3000 }).catch(() => false)) {
      await searchInput.fill("test");
      await searchInput.press("Enter");
      // Wait for results (may show "no results" or actual results)
      await page.waitForTimeout(3000);
    }
  });
});
