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
import groovy.json.JsonSlurper as JsonSlurper


post_register = WS.sendRequestAndVerify(findTestObject('Postman/document status', [('baseUrlApiMerchant') : GlobalVariable.baseUrlApiMerchant]))

String apiPostRegisterResponse = post_register.getResponseText()

println('API Post Register Response : ' + apiPostRegisterResponse)

def slurper = new JsonSlurper()

def result = slurper.parseText(post_register.getResponseBodyContent())

def token = result.token

GlobalVariable.token = token

println('Update token : ' + GlobalVariable.token)

WS.sendRequestAndVerify(findTestObject('Postman/status-user-merchant', [('baseUrlApiMerchant') : GlobalVariable.baseUrlApiMerchant, ('userName') : GlobalVariable.userName, ('password') : GlobalVariable.password, 
	('token') : GlobalVariable.token]))

def verifiedCode = WS.verifyResponseStatusCode(sendPost, 200)


