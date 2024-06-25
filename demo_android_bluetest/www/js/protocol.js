
function toBytesInt32(num) {
	const arr = new ArrayBuffer(4);
	const view = new DataView(arr);
	view.setUint32(0, num, false);
	
	return new Uint8Array(arr);
}

function make_ascii_arraybuff(str) {
	let result = [];
	for (var i = 0; i < str.length; ++i) {
		let code = str.charCodeAt(i);

		result = result.concat([code]);
	}
	return result;
}

// TODO!!
async function generateSecureIV(password) {
	var result = 0;
	cordova.plugins.AES256.generateSecureIV(
		password, 
		(r) => {
			result = r;
			logToDom('15 ' + result);
		},
		(e) => {
			result = null;
			logToDom('25 ' + e);
		},
	);
	if (result === 0) {
		await sleep(10);
	}
	while (result === 0) {
		await sleep(50);
	}
	return result;
}

// TODO!!
async function generateSecureKey(password) {
	var result = 0;
	cordova.plugins.AES256.generateSecureKey(
		password, 
		(r) => {
			result = r;
			logToDom('27 ' + result);
		},
		(e) => {
			result = null;
			logToDom('25 ' + e);
		},
	);
	if (result === 0) {
		await sleep(10);
	}
	while (result === 0) {
		await sleep(50);
	}
	return result;
}

async function make_protocol_bleconnect_to_me(data = null, date = null) {
	if (data === null) {
		data = [];
	}
	
	let c_uuid_ble = ble_adapter_info.address.split(":");
	let i = 0;
	while (i < c_uuid_ble.length) {
		data = data.concat(hexToBytes(c_uuid_ble[i]));
		i += 1;
	}
	//logToDom('make_protocol_bleconnect_to_me: ' + data);
	return await make_secure_data(data, null, null, date);
	//return data;
}

async function make_secure_data(data = null, sec_key = null, sec_iv = null, date = null) {
	if (data === null) {
		data = [];
	}
	if (sec_key === null) {
		sec_key = [].concat(bkey_device_uuid);
		sec_key = sec_key.concat(current_device_uuid);
		// BKEY_UUID + CURRENT_UUID
		
		let mdate = make_universal_time(date);
		detect_minuts_reset = mdate.mi;
		
		sec_key = sec_key.concat(toBytesInt32(mdate.sec_time));
		// BKEY_UUID + CURRENT_UUID + UNIVERSAL_TIME
		
		logToDom('sec_key: ' + sec_key + "   " + (bytesToHex(sec_key) + "") + "   " + sec_key.length);
		sec_key = await generateSecureKey(bytesToHex(sec_key));
		if (sec_key === null) {
			return null;
		}
		
		logToDom('rsec_key_len: ' + sec_key.length);
	}else {
		detect_minuts_reset = null;
	}
	if (sec_iv === null) {
		sec_iv = [].concat(bkey_device_uuid);
		sec_iv = sec_iv.concat(current_device_uuid);
		// BKEY_UUID + CURRENT_UUID
		
		logToDom('sec_iv: ' + sec_iv + "   " + (bytesToHex(sec_iv) + "") + "   " + sec_iv.length);
		sec_iv = await generateSecureIV(bytesToHex(sec_iv));
		if (sec_iv === null) {
			return null;
		}
		
		logToDom('rsec_iv_len: ' + sec_iv.length);
	}
	
	logToDom('encrypt...');
	
	data = data.concat(crc16(data));
	
	var result = 0;
	cordova.plugins.AES256.encrypt(sec_key, sec_iv, data,
		(encrypedData) => {
			encrypedData = make_ascii_arraybuff(encrypedData);
			encrypedData.pop(); // 10
			encrypedData.pop(); // 61
			
			logToDom('len: ' + encrypedData.length);
			logToDom('Encrypted Data---- ' + encrypedData);
			
			result = encrypedData;
		}, (error) => {
			logToDom('Error----' + error);
			result = null;
		}
	);
	if (result === 0) {
		await sleep(10);
	}
	while (result === 0) {
		await sleep(50);
	}
	return result;
}
