
/**
 * Convert array of 16 byte values to UUID string format of the form:
 * XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX
 */
var byteToHex = [];
//var beaconIdentGen = 0;
var ident_version = "beaconAtTheMacBooks";
var randnum = Math.floor(256 * Math.random());

for (let i = 0; i < 256; ++i) {
	byteToHex.push((i + 0x100).toString(16).slice(1));
}

function unsafeStringify(arr, offset = 0) {
	// Note: Be careful editing this code!  It's been tuned for performance
	// and works in ways you may not expect. See https://github.com/uuidjs/uuid/pull/434
	return (
		byteToHex[arr[offset + 0]] +
		byteToHex[arr[offset + 1]] +
		byteToHex[arr[offset + 2]] +
		byteToHex[arr[offset + 3]] +
		'-' +
		byteToHex[arr[offset + 4]] +
		byteToHex[arr[offset + 5]] +
		'-' +
		byteToHex[arr[offset + 6]] +
		byteToHex[arr[offset + 7]] +
		'-' +
		byteToHex[arr[offset + 8]] +
		byteToHex[arr[offset + 9]] +
		'-' +
		byteToHex[arr[offset + 10]] +
		byteToHex[arr[offset + 11]] +
		byteToHex[arr[offset + 12]] +
		byteToHex[arr[offset + 13]] +
		byteToHex[arr[offset + 14]] +
		byteToHex[arr[offset + 15]]
	).toLowerCase();
}

function stringify(arr, offset = 0) {
	const uuid = unsafeStringify(arr, offset);
	// Consistency check for valid UUID.  If this throws, it's likely due to one
	// of the following:
	// - One or more input array values don't map to a hex octet (leading to
	// "undefined" in the uuid)
	// - Invalid input values for the RFC `version` or `variant` fields


	return uuid;
}
 

// Convert a hex string to a byte array
function hexToBytes(hex) {
	for (var bytes = [], c = 0; c < hex.length; c += 2)
		bytes.push(parseInt(hex.substr(c, 2), 16));
	return bytes;
}

// Convert a byte array to a hex string
function bytesToHex(bytes) {
	for (var hex = [], i = 0; i < bytes.length; i++) {
		var current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
		hex.push((current >>> 4).toString(16));
		hex.push((current & 0xF).toString(16));
	}
	return hex.join("");
}

function u16ToByteArray(long) {
	// we want to represent the input as a 8-bytes array
	let byteArray = [0, 0];

	for ( let index = 0; index < byteArray.length; index ++ ) {
		let byte = long & 0xff;
		byteArray [ index ] = byte;
		long = (long - byte) / 256 ;
	}

	return byteArray;
};
 
function byteArrayToU16(/*byte[]*/byteArray) {
	let invers = byteArray[1];
	byteArray[1] = byteArray[0];
	byteArray[0] = invers;
	
	let value = 0;
	for ( let i = byteArray.length - 1; i >= 0; i--) {
		value = (value * 256) + byteArray[i];
	}

	return value;
}

function splitAt(index, xs) {
	let result = [xs.slice(0, index), xs.slice(index)];
	while (result.length != 2) {
		result = result.concat([[]]);
	}
	if (result[0] == undefined || result[0] == null) {
		result[0] = [];
	}
	if (result[1] == undefined || result[1] == null) {
		result[1] = [];
	}
	return result;
}

function make_arraybeacons_rw_region(to_uuid = bkey_device_uuid, c_uuid = current_device_uuid, message = null) {
	if (message == undefined || message == null){
		message = [];
	}
	
	let result = [];
	
	let num_message = 0;
	while (true) {
		message = splitAt(8, message);
		let beacon = make_beacon_rwregion(
			to_uuid, c_uuid,
			
			num_message,
			message[0],
			null
		);
		result = result.concat(beacon);
		message = message[1];
		if (message.length == 0) {
			break;
		}
		num_message += 1;
	}
	return result;
}

function make_beacon_rwregion(to_uuid = bkey_device_uuid, c_uuid = current_device_uuid, num_message = 0, mdata=null, len=null) {
	if (mdata == undefined || mdata == null){
		mdata = [];
	}
	return rawmake_beacon_region(
		num_message,
		len = mdata.length,
		to_uuid.concat(c_uuid).concat(mdata)
	);
}

function rawmake_beacon_region(num_message = 0, len=null, indata=null) { // data max 8bytes!
	//let len = data.length;
	if (len === null) {
		len = indata.length;
	}
	if (len > 255) {
		logToDom('make_beacon_region invalid: ' + splitat[0]);
		return;
	}
	
	let data = [num_message, len].concat(indata);
	let mhash = crc16(data);
	data = data.concat(u16ToByteArray(mhash));
	
	let identifier = "null";
	//let identifier = ident_version + beaconIdentGen + Math.floor(256 * Math.random());
	//beaconIdentGen += 1;
	let major = 0; // 2bytes (u16)
	let minor = 0; // 2bytes (u16)
	
	let splitat = splitAt(16, data);
	data = splitat[1];
	
	let new_uuid = splitat[0]; // 16bytes
	if (new_uuid.length != 16) {
		while (new_uuid.length != 16) {
			randnum = Math.floor(256 * Math.random());
			new_uuid = new_uuid.concat([randnum]);
		}
		major = Math.floor(256 * Math.random());
		minor = Math.floor(256 * Math.random());
	}else {
		if (data.length > 0) {
			let splitat = splitAt(2, data);
			data = splitat[1];
			
			//logToDom('rawmajor: ' + splitat[0]);
			
			while (splitat[0].length != 2) {
				splitat[0] = splitat[0].concat([0]);
			}
			major = byteArrayToU16(splitat[0]);
			
			if (data.length > 0) {
				let splitat = splitAt(2, data);
				data = splitat[1];
				
				//logToDom('rawminor: ' + splitat[0]);
				
				while (splitat[0].length != 2) {
					splitat[0] = splitat[0].concat([0]);
				}
				minor = byteArrayToU16(splitat[0]);
			}else {
				minor = Math.floor(256 * Math.random());
			}
		}else {
			major = Math.floor(256 * Math.random());
		}
	}
	if (data.length > 0) {
		logToDom('INVALID GENBEACON DATA, LENGTH > ' + data.length);
	}
	
	//major = byteArrayToU16([35, 49]);
	//minor = byteArrayToU16([0, 111]);
	
	//logToDom('');
	logToDom('major: ' + major);
	logToDom('minor: ' + minor);
	logToDom('new_uuid: ' + new_uuid);
	logToDom('uuid_str: ' + transform_arraytouuid(new_uuid));
	logToDom('');
	
	new_uuid = transform_arraytouuid(new_uuid);
	
	let beacon = new cordova.plugins.locationManager.BeaconRegion(
		identifier, 
		new_uuid, 
		major, 
		minor
	);
	return {
		beacon: beacon,
		data: {
			uuid: new_uuid,
			major: major,
			minor: minor,
		}
	};
}

