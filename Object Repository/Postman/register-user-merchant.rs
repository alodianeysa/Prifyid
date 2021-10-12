<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>register-user-merchant</name>
   <tag></tag>
   <elementGuidId>cf0d8348-9625-4145-9557-cd835a00ad15</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;aracktdy+633@gmail.com\&quot;,\n  \&quot;phone\&quot;: \&quot;0851234792004\&quot;,\n  \&quot;selfie\&quot;: \&quot;C:\\Users\\Neysa\\Downloads\\victor-grabarczyk-doxeYXgrwCI-unsplash.jpg\&quot;,\n  \&quot;ktp\&quot;: \&quot;C:\\Users\\Neysa\\Downloads\\fgnsfac6zmr8wi6yb7re.jpg\&quot;,\n  \&quot;nik\&quot;: \&quot;123456564454644\&quot;,\n  \&quot;nama\&quot;: \&quot;Tiffany Kumala\&quot;,\n  \&quot;tanggalLahir\&quot;: \&quot;1983-01-02\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Merchant-Key</name>
      <type>Main</type>
      <value>699cba4693s845dld2g3</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrlApiMerchant}/registration</restUrl>
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
      <id>8fb0896c-d163-4666-a231-b447f84140b4</id>
      <masked>false</masked>
      <name>baseUrlApiMerchant</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userName</defaultValue>
      <description></description>
      <id>68c04292-500a-436b-ba49-89557e553e23</id>
      <masked>false</masked>
      <name>userName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>aa1ed4de-add4-4855-97a8-880aa40729d6</id>
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
