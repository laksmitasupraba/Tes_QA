import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.putraprima.id/')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/h5_Kalkulator Faktorial'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Qa Academy                             _45f899'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Laravel v8.72.0 (PHP v8.0.10)'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.verifyElementClickable(findTestObject('OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487', 
        [('variable') : '']))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Qa Academy                             _45f899'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Laravel v8.72.0 (PHP v8.0.10)'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.verifyElementClickable(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.putraprima.id/')

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4!')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '3')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari 3 adalah 6'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari 4 adalah 24'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '-4')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari -4 adalah 1'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '-3')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '!')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '444')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '44')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.closeBrowser()

WebUI.verifyElementClickable(findTestObject('OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.putraprima.id/')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/h5_Kalkulator Faktorial'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Qa Academy                             _45f899'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Laravel v8.72.0 (PHP v8.0.10)'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.verifyElementClickable(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Qa Academy                             _45f899'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Kalkulator Faktorial                   _d8eee7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Laravel v8.72.0 (PHP v8.0.10)'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/html_Faktoriall                    katalonf_451667'))

WebUI.verifyElementClickable(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.putraprima.id/')

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4!')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '3')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari 3 adalah 6'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari 4 adalah 24'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '-4')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Faktorial dari -4 adalah 1'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '-3')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '!')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '444')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Please enter an integer'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '44')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/form_Hitung                                _30ec92'))

WebUI.setText(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/input_Kalkulator Faktorial_input'), 
    '4')

'notif input salah tidak muncul'
WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'))

WebUI.verifyElementClickable(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/button_Hitung                              _be4487'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.openBrowser('')

WebUI.navigateToUrl('https://qa.putraprima.id/')

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

WebUI.click(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/a_Terms Of Service'))

WebUI.closeBrowser()

WebUI.verifyElementClickable(findTestObject('Object Repository/OR_kalkulator_faktorial/Page_Faktoriall/div_Terms Of Service                     Pr_cbc5e7'))

