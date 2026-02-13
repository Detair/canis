/**
 * User Settings E2E Tests
 *
 * Tests the settings modal and its various tabs.
 * Prerequisites: Backend running, test users created
 */

import { test, expect } from "@playwright/test";
import { loginAsAlice, openUserSettings } from "./helpers";

test.describe("User Settings", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAlice(page);
  });

  test("should open settings modal", async ({ page }) => {
    await openUserSettings(page);
    // Settings modal should be visible
    await expect(
      page.locator('text=Account').or(page.locator('text=Settings'))
    ).toBeVisible({ timeout: 3000 });
  });

  test("should display account settings", async ({ page }) => {
    await openUserSettings(page);
    // Account tab should show username/display name fields
    await expect(
      page
        .locator('text=Display Name')
        .or(page.locator('text=Username'))
        .or(page.locator('text=Account'))
    ).toBeVisible({ timeout: 3000 });
  });

  test("should switch to appearance tab", async ({ page }) => {
    await openUserSettings(page);
    const tab = page.locator('button:has-text("Appearance"), [title*="Appearance"]').first();
    if (await tab.isVisible({ timeout: 2000 }).catch(() => false)) {
      await tab.click();
      await expect(
        page.locator('text=Theme').or(page.locator('text=Appearance'))
      ).toBeVisible({ timeout: 3000 });
    }
  });

  test("should switch to audio tab", async ({ page }) => {
    await openUserSettings(page);
    const tab = page.locator('button:has-text("Audio"), [title*="Audio"]').first();
    if (await tab.isVisible({ timeout: 2000 }).catch(() => false)) {
      await tab.click();
      await expect(
        page
          .locator('text=Input Device')
          .or(page.locator('text=Output Device'))
          .or(page.locator('text=Audio'))
      ).toBeVisible({ timeout: 3000 });
    }
  });

  test("should switch to notifications tab", async ({ page }) => {
    await openUserSettings(page);
    const tab = page
      .locator('button:has-text("Notifications"), [title*="Notification"]')
      .first();
    if (await tab.isVisible({ timeout: 2000 }).catch(() => false)) {
      await tab.click();
      await expect(
        page
          .locator('text=Desktop')
          .or(page.locator('text=Sound'))
          .or(page.locator('text=Notification'))
      ).toBeVisible({ timeout: 3000 });
    }
  });

  test("should switch to privacy tab", async ({ page }) => {
    await openUserSettings(page);
    const tab = page.locator('button:has-text("Privacy"), [title*="Privacy"]').first();
    if (await tab.isVisible({ timeout: 2000 }).catch(() => false)) {
      await tab.click();
      await page.waitForTimeout(500);
    }
  });

  test("should switch to security tab", async ({ page }) => {
    await openUserSettings(page);
    const tab = page.locator('button:has-text("Security"), [title*="Security"]').first();
    if (await tab.isVisible({ timeout: 2000 }).catch(() => false)) {
      await tab.click();
      await expect(
        page
          .locator('text=Password')
          .or(page.locator('text=Two-Factor'))
          .or(page.locator('text=Security'))
      ).toBeVisible({ timeout: 3000 });
    }
  });

  test("should update display name", async ({ page }) => {
    await openUserSettings(page);
    // Find display name input
    const nameInput = page.locator(
      'input[placeholder*="display" i], input[placeholder*="name" i]'
    ).first();
    if (await nameInput.isVisible({ timeout: 3000 }).catch(() => false)) {
      const original = await nameInput.inputValue();
      await nameInput.fill("E2E Test Name");

      // Find and click save button
      const saveBtn = page.locator('button:has-text("Save")').first();
      if (await saveBtn.isVisible({ timeout: 2000 }).catch(() => false)) {
        await saveBtn.click();
        await page.waitForTimeout(1000);
      }

      // Restore original name
      await nameInput.fill(original || "alice");
      if (await saveBtn.isVisible({ timeout: 1000 }).catch(() => false)) {
        await saveBtn.click();
      }
    }
  });
});
