<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Kalkulator Faktorial                  _1e85c7</name>
   <tag></tag>
   <elementGuidId>56a06895-986c-4a01-a29e-b7dee6e77995</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        
            
                
                    Kalkulator Faktorial
                    
                        
                            
                                
                            
                            
                                
                                    Hitung
                                        Faktorial
                                
                            
                            
                                Please enter an integer
                            
                        
                    
                

                
                     Terms Of Service
                     Privacy Policy
                
                
                    © Qa Academy
                    
                        document.write(new Date().getFullYear())
                    2022 -
                    
                        document.write(new Date().getFullYear())
                    2022
                
                
                    Laravel v8.72.0 (PHP v8.0.10)
                
            
        
    
    
    
        $(&quot;document&quot;).ready(function() {
            $(&quot;#hitung&quot;).click(function(e) {
                e.preventDefault();
                var input = $('#input').val();
                if (input != parseInt(input, 10)) {
                    $(&quot;#input&quot;).css('border', 'solid 2px red');
                    $(&quot;#result&quot;).text('Please enter an integer')
                    $(&quot;#result&quot;).css('color', 'red');
                } else {
                    $(&quot;#input&quot;).css('border', 'solid 1px #ccc');
                    var callDetails = {
                        type: 'POST',
                        url: '/faktorial',
                        data: {
                            'number': input
                        }
                    };
                    $.ajax(callDetails).done(function(factorial) {
                        console.log('Done Calling');
                        console.log(factorial);
                        $(&quot;#result&quot;).css('color', 'black');
                        $(&quot;#result&quot;).text('Faktorial dari ' + input + ' adalah: ' + factorial
                            .result);
                    });
                }
            });
        });
    



/html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
        
            
                
                    Kalkulator Faktorial
                    
                        
                            
                                
                            
                            
                                
                                    Hitung
                                        Faktorial
                                
                            
                            
                                Please enter an integer
                            
                        
                    
                

                
                     Terms Of Service
                     Privacy Policy
                
                
                    © Qa Academy
                    
                        document.write(new Date().getFullYear())
                    2022 -
                    
                        document.write(new Date().getFullYear())
                    2022
                
                
                    Laravel v8.72.0 (PHP v8.0.10)
                
            
        
    
    
    
        $(&quot;document&quot;).ready(function() {
            $(&quot;#hitung&quot;).click(function(e) {
                e.preventDefault();
                var input = $(&quot; , &quot;'&quot; , &quot;#input&quot; , &quot;'&quot; , &quot;).val();
                if (input != parseInt(input, 10)) {
                    $(&quot;#input&quot;).css(&quot; , &quot;'&quot; , &quot;border&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;solid 2px red&quot; , &quot;'&quot; , &quot;);
                    $(&quot;#result&quot;).text(&quot; , &quot;'&quot; , &quot;Please enter an integer&quot; , &quot;'&quot; , &quot;)
                    $(&quot;#result&quot;).css(&quot; , &quot;'&quot; , &quot;color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;red&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;#input&quot;).css(&quot; , &quot;'&quot; , &quot;border&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;solid 1px #ccc&quot; , &quot;'&quot; , &quot;);
                    var callDetails = {
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot; , &quot;'&quot; , &quot;/faktorial&quot; , &quot;'&quot; , &quot;,
                        data: {
                            &quot; , &quot;'&quot; , &quot;number&quot; , &quot;'&quot; , &quot;: input
                        }
                    };
                    $.ajax(callDetails).done(function(factorial) {
                        console.log(&quot; , &quot;'&quot; , &quot;Done Calling&quot; , &quot;'&quot; , &quot;);
                        console.log(factorial);
                        $(&quot;#result&quot;).css(&quot; , &quot;'&quot; , &quot;color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;black&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#result&quot;).text(&quot; , &quot;'&quot; , &quot;Faktorial dari &quot; , &quot;'&quot; , &quot; + input + &quot; , &quot;'&quot; , &quot; adalah: &quot; , &quot;'&quot; , &quot; + factorial
                            .result);
                    });
                }
            });
        });
    



/html[1]/body[1]&quot;) or . = concat(&quot;
    
        
            
                
                    Kalkulator Faktorial
                    
                        
                            
                                
                            
                            
                                
                                    Hitung
                                        Faktorial
                                
                            
                            
                                Please enter an integer
                            
                        
                    
                

                
                     Terms Of Service
                     Privacy Policy
                
                
                    © Qa Academy
                    
                        document.write(new Date().getFullYear())
                    2022 -
                    
                        document.write(new Date().getFullYear())
                    2022
                
                
                    Laravel v8.72.0 (PHP v8.0.10)
                
            
        
    
    
    
        $(&quot;document&quot;).ready(function() {
            $(&quot;#hitung&quot;).click(function(e) {
                e.preventDefault();
                var input = $(&quot; , &quot;'&quot; , &quot;#input&quot; , &quot;'&quot; , &quot;).val();
                if (input != parseInt(input, 10)) {
                    $(&quot;#input&quot;).css(&quot; , &quot;'&quot; , &quot;border&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;solid 2px red&quot; , &quot;'&quot; , &quot;);
                    $(&quot;#result&quot;).text(&quot; , &quot;'&quot; , &quot;Please enter an integer&quot; , &quot;'&quot; , &quot;)
                    $(&quot;#result&quot;).css(&quot; , &quot;'&quot; , &quot;color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;red&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;#input&quot;).css(&quot; , &quot;'&quot; , &quot;border&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;solid 1px #ccc&quot; , &quot;'&quot; , &quot;);
                    var callDetails = {
                        type: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                        url: &quot; , &quot;'&quot; , &quot;/faktorial&quot; , &quot;'&quot; , &quot;,
                        data: {
                            &quot; , &quot;'&quot; , &quot;number&quot; , &quot;'&quot; , &quot;: input
                        }
                    };
                    $.ajax(callDetails).done(function(factorial) {
                        console.log(&quot; , &quot;'&quot; , &quot;Done Calling&quot; , &quot;'&quot; , &quot;);
                        console.log(factorial);
                        $(&quot;#result&quot;).css(&quot; , &quot;'&quot; , &quot;color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;black&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#result&quot;).text(&quot; , &quot;'&quot; , &quot;Faktorial dari &quot; , &quot;'&quot; , &quot; + input + &quot; , &quot;'&quot; , &quot; adalah: &quot; , &quot;'&quot; , &quot; + factorial
                            .result);
                    });
                }
            });
        });
    



/html[1]/body[1]&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
