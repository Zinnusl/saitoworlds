build:
	# wasm-pack build --target bundler
	trunk build 
	cd www && npm run build
	rm -R ../saito-lite-rust/mods/saitoworlds
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" dist www
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" mods/saitoworlds ../saito-lite-rust/mods/
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" www ../saito-lite-rust/mods/saitoworlds
	mv ../saito-lite-rust/mods/saitoworlds/www ../saito-lite-rust/mods/saitoworlds/web
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" www/dist/* ../saito-lite-rust/mods/saitoworlds/web
	rm -R www/dist
	rm -R pkg

copy_to_wsl:
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" --exclude "target" /mnt/d/Entwicklung/rust/saitoworlds/ .

.PHONY: build
.PHONY: copy_to_wsl
