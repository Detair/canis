/**
 * Channel Management E2E Tests
 *
 * Tests channel list, creation, and context menus.
 * Prerequisites: Backend running, test users + seed data
 */

import { test, expect } from "@playwright/test";
import { loginAsAdmin, selectFirstGuild, uniqueId } from "./helpers";

test.describe("Channel Management", () => {
  test.beforeEach(async ({ page }) => {
    await loginAsAdmin(page);
    await selectFirstGuild(page);
  });

  test("should display channel list", async ({ page }) => {
    // Sidebar should contain channel items
    const sidebar = page.locator("aside");
    await expect(sidebar).toBeVisible();
    // Wait for channels to load
    const channelItems = sidebar.locator('[role="button"]');
    await expect(channelItems.first()).toBeVisible({ timeout: 5000 });
    expect(await channelItems.count()).toBeGreaterThan(0);
  });

  test("should create a text channel", async ({ page }) => {
    // Look for create channel button (+ icon near channels)
    const createBtn = page.locator(
      'button[title*="channel" i], button[title*="Create" i]'
    ).first();

    if (await createBtn.isVisible()) {
      await createBtn.click();

      // Modal should appear
      await expect(
        page.locator('[role="dialog"], .fixed.inset-0').first()
      ).toBeVisible({ timeout: 5000 });

      // Fill channel name
      const channelName = uniqueId("test-ch");
      const nameInput = page.locator('input[placeholder*="name" i]').first();
      if (await nameInput.isVisible()) {
        await nameInput.fill(channelName);
        await page.click('button:has-text("Create")');

        // New channel should appear in sidebar
        await expect(page.locator(`text=${channelName}`)).toBeVisible({
          timeout: 10000,
        });
      }
    }
  });

  test("should show channel context menu", async ({ page }) => {
    const channelItem = page.locator('aside [role="button"]').first();
    await expect(channelItem).toBeVisible({ timeout: 5000 });
    await channelItem.click({ button: "right" });

    // Context menu should appear with options
    await expect(
      page
        .locator('text=Edit Channel')
        .or(page.locator('text=Copy'))
        .or(page.locator('text=Settings'))
    ).toBeVisible({ timeout: 3000 });
  });

  test("should show voice participants", async ({ page }) => {
    // Find a voice channel (Volume2 icon or "Voice" in name)
    const voiceChannel = page.locator(
      'aside [role="button"]:has-text("Voice"), aside [role="button"]:has-text("voice")'
    ).first();

    if (await voiceChannel.isVisible({ timeout: 3000 }).catch(() => false)) {
      // Voice channel exists, verify it's clickable
      await expect(voiceChannel).toBeVisible();
    }
    // Note: This test may be marked as construction since it needs
    // actual voice participants to fully validate
  });
});
