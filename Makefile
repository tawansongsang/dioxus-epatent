SHELL=/home/tawan/.cargo/bin/nu

tx:
	npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch

dx:
	dx serve --hot-reload 
