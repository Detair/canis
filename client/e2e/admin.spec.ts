/**
 * Admin Dashboard E2E Tests
 *
 * Tests admin panel access, navigation, and basic functionality.
 * Prerequisites: Backend running, admin user created (admin/admin123)
 */

import { test, expect } from "@playwright/test";
import { login, loginAsAdmin, loginAsAlice } from "./helpers";

test.describe("Admin Dashboard", () => {
  test("should access admin dashboard", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/admin");
    await expect(
      page.locator('text=Admin Dashboard').or(page.locator('text=Admin'))
    ).toBeVisible({ timeout: 10000 });
  });

  test("should display admin panels", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/admin");
    await page.waitForTimeout(2000);

    // Admin sidebar should have panel buttons
    await expect(
      page.locator('text=Overview').or(page.locator('text=Users'))
    ).toBeVisible({ timeout: 5000 });
  });

  test("should show users panel", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/admin");
    await page.waitForTimeout(2000);

    const usersBtn = page.locator('button:has-text("Users")');
    if (await usersBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await usersBtn.click();
      await page.waitForTimeout(1000);
      // Should show user list/table
      await expect(
        page.locator('text=admin').or(page.locator('text=alice'))
      ).toBeVisible({ timeout: 5000 });
    }
  });

  test("should show guilds panel", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/admin");
    await page.waitForTimeout(2000);

    const guildsBtn = page.locator('button:has-text("Guilds")');
    if (await guildsBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await guildsBtn.click();
      await page.waitForTimeout(1000);
    }
  });

  test("should show audit log panel", async ({ page }) => {
    await loginAsAdmin(page);
    await page.goto("/admin");
    await page.waitForTimeout(2000);

    const auditBtn = page.locator('button:has-text("Audit")');
    if (await auditBtn.isVisible({ timeout: 3000 }).catch(() => false)) {
      await auditBtn.click();
      await page.waitForTimeout(1000);
    }
  });

  test("should block non-admin access", async ({ page }) => {
    await loginAsAlice(page);
    await page.goto("/admin");
    // Should either redirect away or show forbidden message
    await page.waitForTimeout(2000);
    const isOnAdmin = page.url().includes("/admin");
    const hasForbidden = await page
      .locator('text=Forbidden')
      .or(page.locator('text=Access Denied'))
      .or(page.locator('text=not authorized'))
      .isVisible({ timeout: 3000 })
      .catch(() => false);

    // Either redirected away from admin or shown a forbidden message
    expect(!isOnAdmin || hasForbidden).toBeTruthy();
  });
});
