<!DOCTYPE html>
<head>
	<link href="/static/index.css" rel="stylesheet"></link>
	 <script src="static/jquery.js" type="text/javascript"></script>
	 <script src="static/playlists.js" type="text/javascript"></script>
</head>
<div class="topnav">
	<a href="{{LOGOUT_URL}}" class="topnav_link">Log Out</a>
	<a href="{{CONFIG_URL}}" class="topnav_link">Settings</a>
	<a href="/" class="topnav_active">Home</a>
	<a href="playlists" class="topnav_link">Playlists</a>
</div>
<div class="make_playlist">
	<input type="text" id="set_playlist_name">Playlist Name</input>
	<button id="submit_playlist">Submit</button>
</div>
<div class="body">
{% for vid in videos %}
	<div class="video_div">
		<a href="{{vid.html_url}}">
			<img class="video_img" src="{{vid.thumb_url}}"/>
		</a>
		<button class="playlist_add" id="{{vid.name}}">
			Add to Playlist	
		</button>
	</div>
{% endfor %}
</div>


