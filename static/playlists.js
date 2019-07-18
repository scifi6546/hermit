console.log("hello world!")
function text(){
	$.ajax("/api/playlist_get").done(function(data){console.log(data)})
}
