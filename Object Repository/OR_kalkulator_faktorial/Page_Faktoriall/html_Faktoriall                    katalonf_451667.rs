<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Faktoriall                    katalonf_451667</name>
   <tag></tag>
   <elementGuidId>6e3a123b-3cdb-46a8-b061-eec9a74eeb45</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    

    Faktoriall

    
    
    
    

    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}


    
        
            
                
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
    



/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;
    
    

    Faktoriall

    
    
    
    

    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}


    
        
            
                
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
    



/html[1]&quot;) or . = concat(&quot;
    
    

    Faktoriall

    
    
    
    

    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}


    
        
            
                
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
    



/html[1]&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
