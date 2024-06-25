
var status_div = document.getElementById('status');

var count_message_logtodom = 0;
function logToDom(message) {
	let e = document.createElement('label');
	e.innerText = message;

	let br = document.createElement('br');
	let br2 = document.createElement('br');
	document.body.appendChild(e);
	document.body.appendChild(br);
	document.body.appendChild(br2);
	
	//window.scrollTo(0, window.document.height);
	
	count_message_logtodom += 1;
}

function set_statustxt(txt) {
	status_div.innerText = txt;
}
