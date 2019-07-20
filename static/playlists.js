var clicked_buttons=[]
function text(){
	$.ajax("/api/playlist_get").done(function(data){console.log(data)})
}
$(document).ready(function(){
	$(".playlist_add").on('click',function(test){
		id=test.target.id;
		console.log("clicked");
		found=false;
		for(var i=0;i<clicked_buttons.length;i++){
			console.log(clicked_buttons[i]);
			if(clicked_buttons[i]===id){
				clicked_buttons.splice(i,1);
				console.log("found");
				$(test.target).parent().css("background-color","#0000");
				found=true;
			}
		}
		if(found==false){
			clicked_buttons.push(id);
			console.log(id)
			console.log($(test.target));
			$(test.target).parent().css("background-color","#ffcf00");
		}
		if(clicked_buttons.length>0){
			$(".make_playlist").slideDown(500);
		}
		console.log(clicked_buttons.length)
		if(clicked_buttons.length==0){
			console.log("slid up");
			$(".make_playlist").slideUp(500);
		}
	});
	$("#submit_playlist").on('click',function(){
		name=$("#set_playlist_name")[0].value;
		console.log(name);
		temp_data={action:"make_playlist",playlist_name:name,videos:clicked_buttons}
		console.log(temp_data)
		temp_json=JSON.stringify(temp_data);
		console.log(temp_json);
		$.ajax({method:"POST",url:"/api/playlist_post",data:temp_json})
	});
	$(".make_playlist").slideUp(0);

});
