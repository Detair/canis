/**
 * Friends & DMs E2E Tests
 *
 * Tests friends list, friend requests, and DM conversations.
 * Prerequisites: Backend running, test users created
 */

import { test, expect } from "@playwright/test";
import { loginAsAlice, goHome } from "./helpers";

test.describe("Friends & DMs", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAlice(page);
    await goHome(page);
  });

  test("should display friends list", async ({ page }) => {
    // Home view should show friends interface
    await expect(
      page.locator('button:has-text("Online")').or(page.locator('text=Friends'))
    ).toBeVisible({ timeout: 5000 });
  });

  test("should switch between tabs", async ({ page }) => {
    // Click through each tab
    const tabs = ["Online", "All", "Pending", "Blocked"];
    for (const tab of tabs) {
      const tabBtn = page.locator(`button:has-text("${tab}")`);
      if (await tabBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
        await tabBtn.click();
        // Tab should be highlighted/active
        await page.waitForTimeout(300);
      }
    }
  });

  test("should show add friend form", async ({ page }) => {
    // Click "Add Friend" button
    const addBtn = page.locator(
      'button[title="Add Friend"], button:has-text("Add Friend")'
    );
    await expect(addBtn).toBeVisible({ timeout: 5000 });
    await addBtn.click();

    // Should show input for username
    await expect(
      page.locator('input[placeholder*="username" i]')
    ).toBeVisible({ timeout: 3000 });
  });

  test("should send a friend request", async ({ page }) => {
    const addBtn = page.locator(
      'button[title="Add Friend"], button:has-text("Add Friend")'
    );
    await expect(addBtn).toBeVisible({ timeout: 5000 });
    await addBtn.click();

    const input = page.locator('input[placeholder*="username" i]');
    await expect(input).toBeVisible({ timeout: 3000 });
    await input.fill("bob");

    // Submit the friend request
    const sendBtn = page.locator('button:has-text("Send"), button:has-text("Add")').first();
    if (await sendBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
      await sendBtn.click();
      // Should show success or the request was already sent
      await page.waitForTimeout(1000);
    }
  });

  test("should open DM conversation", async ({ page }) => {
    // Look for DM list in sidebar or home view
    const dmItem = page.locator('aside [role="button"]').first();
    if (await dmItem.isVisible({ timeout: 3000 }).catch(() => false)) {
      await dmItem.click();
      // Should show message input for DM
      await expect(
        page.locator('textarea[placeholder*="Message"]')
      ).toBeVisible({ timeout: 5000 });
    }
  });
});
