<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>document status</name>
   <tag></tag>
   <elementGuidId>b3f3b513-905d-45d9-9d88-6e8e72003e00</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Merchant-Key</name>
      <type>Main</type>
      <value>699cba4693s845dld2g3</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrlApiMerchant}/document/status/:docToken?docToken=${docToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrlApiMerchant</defaultValue>
      <description></description>
      <id>19b32328-b069-41e9-899e-35aefb8fce08</id>
      <masked>false</masked>
      <name>baseUrlApiMerchant</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.docToken</defaultValue>
      <description></description>
      <id>7c4275b2-4def-4456-a2f9-f8b255eb0338</id>
      <masked>false</masked>
      <name>docToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userName</defaultValue>
      <description></description>
      <id>22cb0afe-df62-4662-a18c-f12d823f4b7c</id>
      <masked>false</masked>
      <name>userName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>984f15b9-df84-4ce5-91c6-085e74bdce40</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
