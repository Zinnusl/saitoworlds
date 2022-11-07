build:
	wasm-pack build --target bundler
	cd www && npm run build
	rm -R ../saito-lite/mods/saitoworlds
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" mods/saitoworlds ../saito-lite/mods/
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" www ../saito-lite/mods/saitoworlds
	mv ../saito-lite/mods/saitoworlds/www ../saito-lite/mods/saitoworlds/web
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" www/dist/* ../saito-lite/mods/saitoworlds/web
	rm -R www/dist
	rm -R pkg

copy_to_wsl:
	rsync -rupE --exclude ".*" --exclude "node_modules" --exclude "LICENSE*" --exclude "*.json" --exclude "README*" --exclude "dist" --exclude "target" /mnt/d/Entwicklung/rust/saitoworlds/ .

.PHONY: build
.PHONY: copy_to_wsl
