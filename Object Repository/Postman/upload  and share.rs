<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>upload  and share</name>
   <tag></tag>
   <elementGuidId>92fcfd23-b934-4079-b898-02be2282c4cb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;documentTitle&quot;,
      &quot;value&quot;: &quot;tes callback doc&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;docType&quot;,
      &quot;value&quot;: &quot;Parallel&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;owner&quot;,
      &quot;value&quot;: &quot;{\n  \&quot;privyId\&quot;: \&quot;TES001\&quot;,\n  \&quot;enterpriseToken\&quot;: \&quot;41bc84b42c8543daf448d893c255be1dbdcc722e\&quot;\n}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;document&quot;,
      &quot;value&quot;: &quot;C:\\Users\\Neysa\\Downloads\\invoice_#230921207.pdf&quot;,
      &quot;type&quot;: &quot;file&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;[{\n\t\t\t\&quot;privyId\&quot;:\&quot;UAT013\&quot;,\n\t\t\t\&quot;type\&quot;:\&quot;Signer\&quot;,\&quot;enterpriseToken\&quot;: \&quot;\&quot;\n\t\t}\n]\n\t&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;templateId&quot;,
      &quot;value&quot;: &quot;B&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Merchant-Key</name>
      <type>Main</type>
      <value>699cba4693s845dld2g3</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrlApiMerchant}/document/upload</restUrl>
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
      <id>8a4b5429-f8ca-4ee4-8b27-986ba18741d6</id>
      <masked>false</masked>
      <name>baseUrlApiMerchant</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userName</defaultValue>
      <description></description>
      <id>d0af23ed-386c-4e8c-82ff-a35180688c86</id>
      <masked>false</masked>
      <name>userName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>d38d1802-6950-4793-8ae4-74beb7bf8080</id>
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
