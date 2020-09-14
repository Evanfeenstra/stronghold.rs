(function() {

    var application = 'com.iota.stronghold';

    function list_command(){
        var pwd = getel('list-pwd').value
        if(!pwd) return alert('enter password')
        sendMsg({
            cmd:'list',
            password:pwd
        })
    }
    onclick('list-button', list_command)
    onenter('list-pwd', list_command)

    function encrypt_command(){
        var plaintext = getel('encrypt-plaintext').value
        var pwd = getel('encrypt-pwd').value
        if(!plaintext || !pwd) return alert('no plaintext or password')
        sendMsg({
            cmd:'encrypt',
            password:pwd,
            plaintext:plaintext,
        })
    }
    onclick('encrypt-button', encrypt_command)
    onenter('encrypt-pwd', encrypt_command)
    oninput('encrypt-pwd',function(e){
        var val = e.target.value
        getel('list-pwd').value = val
        getel('decrypt-pwd').value = val
    })

    function decrypt_command(){
        var id = getel('decrypt-id').value
        var pwd = getel('decrypt-pwd').value
        if(!id || !pwd) return alert('no id or password')
        sendMsg({
            cmd:'decrypt',
            id:id,
            password:pwd,
        })
    }
    onclick('decrypt-button', decrypt_command)
    onenter('decrypt-pwd', decrypt_command)

    function sendMsg(json){
        var resultEl = getel(json.cmd+'-res')
        resultEl.innerHTML = ''
        var btnEl = getel(json.cmd+'-button')
        btnEl.disabled=true

        console.log('chrome.runtime.sendNativeMessage:',json);
        chrome.runtime.sendNativeMessage(application, json, function(result){
            console.log('=>',result)
            resultEl.innerHTML = JSON.stringify(result,null,2)
            btnEl.disabled=false
        });
    }

    function getel(el){
        return document.getElementById(el)
    }
    function onclick(el, callback) {
        getel(el).addEventListener('click', callback)
    }
    function onenter(el, callback) {
        getel(el).addEventListener('keydown', function(e){
            if(e.key==='Enter') callback()
        })
    }
    function oninput(el, callback) {
        getel(el).addEventListener('input', callback)
    }

})();


