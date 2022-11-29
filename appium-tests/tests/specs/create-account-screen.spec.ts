import CreatePinScreen from "../screenobjects/CreatePinScreen"
import CreateAccountScreen from "../screenobjects/CreateAccountScreen"
import EnterPinScreen from "../screenobjects/EnterPinScreen"
import UplinkMainScreen from "../screenobjects/UplinkMainScreen"

describe("Create Account on Uplink Desktop", async () => {
  before(async () => {
    await CreatePinScreen.waitForIsShown(true)
  })

  after(async () => {
    await driver.deleteSession()
  })

  it("Assert Create PIN screen texts", async () => {
    await expect(CreatePinScreen.headerText).toHaveTextContaining(
      "Create a Pin",
    )
    await expect(CreatePinScreen.subtitleText).toHaveTextContaining(
      "Choose a 4-6 digit pin to secure your account.",
    )
  })

  it("Attempt to use an empty PIN", async () => {
    await (await CreatePinScreen.pinInput).addValue("\n")
    await expect(await CreatePinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with less than 4 characters", async () => {
    await (await CreatePinScreen.pinInput).setValue("123" + "\n")
    await expect(await CreatePinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with more than 6 characters and assert error message", async () => {
    await (await CreatePinScreen.pinInput).setValue("1234567")
    await expect(await CreatePinScreen.maxLengthMessage).toBeDisplayed()
    await expect(await CreatePinScreen.maxLengthMessage).toHaveTextContaining(
      "Only four to six characters allowed",
    )
  })

  it("Type a valid PIN with 4 characters and go to next page", async () => {
    await (await CreatePinScreen.pinInput).setValue("1234" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
    await driver.reset()
  })

  it("Type a valid PIN with 6 characters and go to next page", async () => {
    await (await CreatePinScreen.pinInput).setValue("123456" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
  })

  it("Assert Create Username screen texts", async () => {
    await expect(CreateAccountScreen.headerText).toHaveTextContaining(
      "Create Account",
    )
    await expect(CreateAccountScreen.subtitleText).toHaveTextContaining(
      "It's free and fast, just tell us what you'd like your username to be.",
    )
  })

  it("Attempt to provide an empty username", async () => {
    await (await CreateAccountScreen.userInput).addValue("\n")
    await expect(await CreateAccountScreen.errorMessage).toBeDisplayed()
    await expect(await CreateAccountScreen.errorMessage).toHaveTextContaining(
      "Username is required",
    )
  })

  it("Attempt to provide a username with less than 4 characters", async () => {
    await (await CreateAccountScreen.userInput).setValue("a" + "\n")
    await expect(await CreateAccountScreen.errorMessage).toBeDisplayed()
    await expect(await CreateAccountScreen.errorMessage).toHaveTextContaining(
      "Username length is invalid",
    )
  })

  it("Attempt to provide a username with less more than 26 characters", async () => {
    await (
      await CreateAccountScreen.userInput
    ).setValue("123456789012345678901234567") // Typing 27 characters
    await expect(await CreateAccountScreen.errorMessage).toBeDisplayed()
    await expect(await CreateAccountScreen.errorMessage).toHaveTextContaining(
      "Maximum username length reached (26)",
    )
  })

  it("Provide a valid username and go to next page", async () => {
    await (await CreateAccountScreen.userInput).setValue("qatest01" + "\n")
    await expect(await UplinkMainScreen.noActiveChatsText).toBeDisplayed()
    await driver.reset()
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  it.skip("Reset app and assert Enter Pin Screen Texts", async () => {
    await expect(EnterPinScreen.headerText).toHaveTextContaining("Enter Pin")
    await expect(EnterPinScreen.subtitleText).toHaveTextContaining(
      "Enter pin to unlock your account.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  it.skip("Enter an empty pin and assert error message", async () => {
    await (await EnterPinScreen.pinInput).addValue("\n")
    await expect(await EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  it.skip("Enter an wrong pin value and assert error message", async () => {
    await (await EnterPinScreen.pinInput).setValue("9999" + "\n")
    await expect(await EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  it.skip("Enter a PIN with more than 6 characters and assert error message", async () => {
    await (await EnterPinScreen.pinInput).setValue("1234567")
    await expect(await EnterPinScreen.maxLengthMessage).toBeDisplayed()
    await expect(await EnterPinScreen.maxLengthMessage).toHaveTextContaining(
      "Only four to six characters allowed",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  it.skip("Enter a valid PIN to be redirected to main screen", async () => {
    await (await EnterPinScreen.pinInput).setValue("123456" + "\n")
    await expect(await UplinkMainScreen.noActiveChatsText).toBeDisplayed()
  })
})
