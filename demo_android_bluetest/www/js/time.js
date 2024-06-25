
function make_point_date() {
	let date = new Date();
	
	return date;
}

function make_universal_time(date = null) {
	if (date === null) {
		date = make_point_date();
	}
	
	let y = date.getUTCFullYear();
	if (y < 2023) {
		return null;
	}
	y -= 2023;
	
	let d = date.getUTCDate();
	let m = date.getUTCMonth()+1;
	let h = date.getUTCHours();
	let mi = make_universal_time_minuts(date);
	//let s = date.getSeconds();
	let s = 0;
	
	let datetime = d + "." + m + "." + y + " " + h + ":" + mi + ":" + s;
	logToDom("datetime: " + datetime);
	
	return {
		datetime: datetime,
		sec_time: (
			(d * 86400) +
			(y * 31536000) +
			(m * 2629746) +
			
			(h * 3600) +
			(mi * 60) +
			(s)
		),
		mi: mi,
	}
}

function make_universal_time_minuts(date = null) {
	if (date === null) {
		date = make_point_date();
	}
	let mi = date.getUTCMinutes();
	mi = (~~(Math.round((mi / 10))))*10;
	return mi;
}
