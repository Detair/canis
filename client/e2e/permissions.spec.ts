/**
 * Permission System E2E Tests
 *
 * Tests the Permission System UI workflows:
 * 1. Role Management - Owner can create/edit roles
 * 2. Security Constraints - @everyone cannot have dangerous permissions
 * 3. Member Role Assignment - Assign roles to members via UI
 * 4. Channel Permission Overrides - Set Allow/Deny overrides per channel
 *
 * Prerequisites: Backend running with seed data (admin, alice, bob)
 */

import { test, expect, type Page } from "@playwright/test";

// Helper: Login as a specific user
async function login(page: Page, username: string, password: string = "password123") {
  await page.goto("/login");
  await page.fill('input[type="text"]', username);
  await page.fill('input[type="password"]', password);
  await page.click('button[type="submit"]');
  // Wait for dashboard to load (sidebar visible)
  await expect(page.locator("aside")).toBeVisible({ timeout: 10000 });
}

// Helper: Open Guild Settings Modal
async function openGuildSettings(page: Page) {
  // Click guild header to open dropdown
  await page.click('[data-testid="guild-header"]');
  // Click settings trigger
  await page.click('[data-testid="settings-trigger"]');
  // Wait for modal to appear
  await expect(page.getByText("Server Settings")).toBeVisible({ timeout: 5000 });
}

// Helper: Navigate to a specific tab in guild settings
async function navigateToTab(page: Page, tabName: "Invites" | "Members" | "Roles") {
  await page.click(`button:has-text("${tabName}")`);
}

// Helper: Open channel settings via context menu
async function openChannelSettings(page: Page, channelName: string) {
  // Find and right-click the channel
  const channel = page.locator(`[data-testid="channel-item"]:has-text("${channelName}")`);
  await channel.click({ button: "right" });
  // Click "Edit Channel" from context menu
  await page.click('text=Edit Channel');
  // Wait for modal
  await expect(page.getByText("Channel Settings")).toBeVisible({ timeout: 5000 });
}

test.describe("Permission System", () => {
  test.describe("Role Management (Owner Flow)", () => {
    test("should create a new role with permissions", async ({ page }) => {
      // Login as admin (owner)
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Roles tab
      await navigateToTab(page, "Roles");

      // Click "New Role" button
      await page.click('button:has-text("New Role")');

      // Fill role name
      await page.fill('input[placeholder="Enter role name..."]', "E2E Test Officer");

      // Select a color (first color preset)
      const colorButtons = page.locator('button[title="No color"]').locator("..").locator("button");
      await colorButtons.nth(1).click(); // Select first color

      // Enable "Manage Messages" permission
      await page.click('text=Manage Messages');

      // Enable "Kick Members" permission
      await page.click('text=Kick Members');

      // Save the role
      await page.click('button:has-text("Create Role")');

      // Verify the role appears in the list
      await expect(page.getByText("E2E Test Officer")).toBeVisible({ timeout: 5000 });

      // Verify permissions count shows "2 permissions"
      await expect(page.getByText("2 permissions")).toBeVisible();
    });

    test("should edit an existing role", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Roles tab
      await navigateToTab(page, "Roles");

      // Wait for roles to load
      await expect(page.getByText("Roles")).toBeVisible();

      // Find a non-@everyone role and click edit (settings icon)
      // First, hover over a role row to make the edit button visible
      const roleRow = page.locator('div:has-text("E2E Test Officer")').first();
      await roleRow.hover();

      // Click the settings/edit button
      await roleRow.locator('button[title="Edit role"]').click();

      // Verify we're in edit mode
      await expect(page.getByText("Edit Role:")).toBeVisible();

      // Toggle a permission (e.g., add "Timeout Members")
      const timeoutCheckbox = page.locator('label:has-text("Timeout Members") input[type="checkbox"]');
      await timeoutCheckbox.click();

      // Save changes
      await page.click('button:has-text("Save Changes")');

      // Verify back on role list
      await expect(page.getByText("E2E Test Officer")).toBeVisible();

      // Verify permissions count updated to "3 permissions"
      await expect(page.getByText("3 permissions")).toBeVisible();
    });
  });

  test.describe("@everyone Security Constraints", () => {
    test("should not display dangerous permissions for @everyone role", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Roles tab
      await navigateToTab(page, "Roles");

      // Find @everyone role and click edit
      const everyoneRow = page.locator('div:has-text("@everyone")').first();
      await everyoneRow.hover();
      await everyoneRow.locator('button[title="Edit role"]').click();

      // Verify we're editing @everyone
      await expect(page.getByText("Edit Role: @everyone")).toBeVisible();

      // Verify dangerous permissions are NOT visible (hidden for @everyone)
      // These should be completely hidden, not just disabled
      await expect(page.locator('label:has-text("Ban Members")')).toBeHidden();
      await expect(page.locator('label:has-text("Manage Server")')).toBeHidden();
      await expect(page.locator('label:has-text("Manage Roles")')).toBeHidden();
      await expect(page.locator('label:has-text("Kick Members")')).toBeHidden();
      await expect(page.locator('label:has-text("Manage Messages")')).toBeHidden();

      // Verify safe permissions ARE visible
      await expect(page.locator('label:has-text("Send Messages")')).toBeVisible();
      await expect(page.locator('label:has-text("Embed Links")')).toBeVisible();
      await expect(page.locator('label:has-text("Create Invite")')).toBeVisible();
    });

    test("should allow modifying safe permissions for @everyone", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Roles tab
      await navigateToTab(page, "Roles");

      // Find @everyone role and click edit
      const everyoneRow = page.locator('div:has-text("@everyone")').first();
      await everyoneRow.hover();
      await everyoneRow.locator('button[title="Edit role"]').click();

      // Toggle "Add Reactions" permission
      const reactionsCheckbox = page.locator('label:has-text("Add Reactions") input[type="checkbox"]');
      const wasChecked = await reactionsCheckbox.isChecked();
      await reactionsCheckbox.click();

      // Save
      await page.click('button:has-text("Save Changes")');

      // Re-open @everyone to verify change persisted
      await everyoneRow.hover();
      await everyoneRow.locator('button[title="Edit role"]').click();

      // Verify the toggle state changed
      const newChecked = await page.locator('label:has-text("Add Reactions") input[type="checkbox"]').isChecked();
      expect(newChecked).not.toBe(wasChecked);

      // Revert the change for test idempotency
      await page.locator('label:has-text("Add Reactions") input[type="checkbox"]').click();
      await page.click('button:has-text("Save Changes")');
    });
  });

  test.describe("Member Role Assignment", () => {
    test("should assign a role to a member via the UI", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Members tab
      await navigateToTab(page, "Members");

      // Wait for members to load
      await expect(page.getByText("members")).toBeVisible({ timeout: 5000 });

      // Find alice's row
      const aliceRow = page.locator('div:has-text("alice")').first();

      // Click "Manage" dropdown
      await aliceRow.locator('button:has-text("Manage")').click();

      // Wait for dropdown to appear
      await expect(page.getByText("Assign Role")).toBeVisible();

      // Check if "E2E Test Officer" role exists and toggle it
      const roleCheckbox = page.locator('label:has-text("E2E Test Officer") input[type="checkbox"]');
      const wasAssigned = await roleCheckbox.isChecked();

      if (!wasAssigned) {
        await roleCheckbox.click();

        // Verify role badge appears on alice
        await expect(aliceRow.locator('span:has-text("E2E Test Officer")')).toBeVisible({ timeout: 3000 });
      } else {
        // Role already assigned, toggle off and verify badge disappears
        await roleCheckbox.click();
        await expect(aliceRow.locator('span:has-text("E2E Test Officer")')).toBeHidden({ timeout: 3000 });
      }
    });

    test("should show role badges on members", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Open Guild Settings
      await openGuildSettings(page);

      // Navigate to Members tab
      await navigateToTab(page, "Members");

      // Verify alice has role badges or "(no roles)" text
      const aliceRow = page.locator('div:has-text("alice")').first();

      // Should show either role badges or "(no roles)"
      const hasRoles = await aliceRow.locator("span").filter({ hasText: /(no roles)/ }).count();
      const hasBadges = await aliceRow.locator('span[style*="background-color"]').count();

      // One of these should be true
      expect(hasRoles > 0 || hasBadges > 0).toBe(true);
    });
  });

  test.describe("Channel Permission Overrides", () => {
    test("should add a role override to a channel", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Wait for channels to load
      await expect(page.locator('[data-testid="channel-item"]').first()).toBeVisible({ timeout: 10000 });

      // Open channel settings via context menu on first channel
      const firstChannel = page.locator('[data-testid="channel-item"]').first();
      await firstChannel.click({ button: "right" });

      // Click "Edit Channel"
      await page.click('text=Edit Channel');

      // Wait for channel settings modal
      await expect(page.getByText("Channel Settings")).toBeVisible({ timeout: 5000 });

      // Navigate to Permissions tab
      await page.click('button:has-text("Permissions")');

      // Click "Add Role" button
      await page.click('button:has-text("Add Role")');

      // Select @everyone from dropdown (or first available role)
      const roleOption = page.locator('button:has-text("@everyone")');
      if (await roleOption.isVisible()) {
        await roleOption.click();
      } else {
        // If @everyone already has override, try another role
        const anyRole = page.locator('[style*="background-color: var(--color-surface-layer2)"] button').first();
        await anyRole.click();
      }

      // Should now be in override editor
      await expect(page.getByText("permissions")).toBeVisible({ timeout: 5000 });

      // Set "Send Messages" to Deny
      const sendMessagesRow = page.locator('div:has-text("Send Messages")').first();
      await sendMessagesRow.locator('label:has-text("Deny") input[type="radio"]').click();

      // Save
      await page.click('button:has-text("Save")');

      // Verify override shows in list with denied count
      await expect(page.locator('text=-1 denied')).toBeVisible({ timeout: 3000 });
    });

    test("should set Allow/Deny/Inherit for channel permissions", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Wait for channels to load
      await expect(page.locator('[data-testid="channel-item"]').first()).toBeVisible({ timeout: 10000 });

      // Open channel settings
      const firstChannel = page.locator('[data-testid="channel-item"]').first();
      await firstChannel.click({ button: "right" });
      await page.click('text=Edit Channel');

      // Navigate to Permissions tab
      await page.click('button:has-text("Permissions")');

      // If there's an existing override, edit it; otherwise add one
      const existingOverride = page.locator('div:has-text("@everyone")').locator('button[title="Edit override"]');
      const editButton = page.locator('button:has([class*="Settings"])').first();

      if (await editButton.isVisible()) {
        await editButton.click();
      } else {
        // Add @everyone override first
        await page.click('button:has-text("Add Role")');
        const everyoneOption = page.locator('button:has-text("@everyone")');
        if (await everyoneOption.isVisible()) {
          await everyoneOption.click();
        }
      }

      // Test the three-state radio buttons
      const embedLinksRow = page.locator('div:has-text("Embed Links")').first();

      // Set to Allow
      await embedLinksRow.locator('label:has-text("Allow") input[type="radio"]').click();
      expect(await embedLinksRow.locator('label:has-text("Allow") input[type="radio"]').isChecked()).toBe(true);

      // Set to Inherit
      await embedLinksRow.locator('label:has-text("Inherit") input[type="radio"]').click();
      expect(await embedLinksRow.locator('label:has-text("Inherit") input[type="radio"]').isChecked()).toBe(true);

      // Set to Deny
      await embedLinksRow.locator('label:has-text("Deny") input[type="radio"]').click();
      expect(await embedLinksRow.locator('label:has-text("Deny") input[type="radio"]').isChecked()).toBe(true);

      // Save
      await page.click('button:has-text("Save")');
    });

    test("should delete a channel override", async ({ page }) => {
      // Login as admin
      await login(page, "admin");

      // Wait for channels to load
      await expect(page.locator('[data-testid="channel-item"]').first()).toBeVisible({ timeout: 10000 });

      // Open channel settings
      const firstChannel = page.locator('[data-testid="channel-item"]').first();
      await firstChannel.click({ button: "right" });
      await page.click('text=Edit Channel');

      // Navigate to Permissions tab
      await page.click('button:has-text("Permissions")');

      // Check if there's an override to delete
      const deleteButton = page.locator('button:has([class*="Trash2"])').first();

      if (await deleteButton.isVisible()) {
        // Count overrides before
        const overrideCountBefore = await page.locator('[style*="background-color: var(--color-surface-layer1)"]').count();

        // Click delete
        await deleteButton.click();

        // Verify override was removed (count decreased or message shown)
        await page.waitForTimeout(500);
        const overrideCountAfter = await page.locator('[style*="background-color: var(--color-surface-layer1)"]').count();

        // Either count decreased or we see the "no overrides" message
        const noOverridesMessage = page.locator('text=No permission overrides');
        expect(overrideCountAfter < overrideCountBefore || await noOverridesMessage.isVisible()).toBe(true);
      }
    });
  });
});
