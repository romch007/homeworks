import { test, expect } from "@playwright/test";

test("create a subject", async ({ page }) => {
  await page.goto("./subjects");

  const createYourFirstSubjectBtn = page.getByRole("button", {
    name: "Create your first subject",
  });
  const newSubjectBtn = page.getByRole("button", {
    name: "New Subject",
  });

  if (await createYourFirstSubjectBtn.isVisible()) {
    await createYourFirstSubjectBtn.click();
  } else {
    await newSubjectBtn.click();
  }

  const subjectName = "Maths";

  await page.getByRole("textbox", { name: "Name Name" }).fill(subjectName);

  await page.getByRole("button", { name: "Change color" }).click();

  // Purple
  await page.locator("div:nth-child(4) > .v-sheet").click();

  await page.getByRole("button", { name: "Create", exact: true }).click();
});
