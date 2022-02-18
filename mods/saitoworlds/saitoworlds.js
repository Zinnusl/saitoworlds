const GameTemplate = require('../../lib/templates/gametemplate');

class SaitoworldsGame extends GameTemplate {

    constructor(app) {
        super(app);

        this.name = "Saitoworlds";
        this.description = "Saitoworlds is a infinite player 4x game played on an infinite hex grid."
        this.publickey = app.wallet.returnPublicKey();

        this.useHUD = 0;
        this.useClock = 1;

        this.wasm = null;

        this.minPlayers = 1;
        this.maxPlayers = 9;
        this.type       = "4x";
        this.description = "An implementation of Eressea for the Saito Blockchain";
        this.categories  = "Fantasy 4x Game";

        return this;
    }

    //
    // manually announce arcade banner support
    //
    respondTo(type) {
        if (super.respondTo(type) != null) {
            return super.respondTo(type);
        }

        if (type == "arcade-carousel") {
            let obj = {};
            obj.background = "/chess/img/arcade/arcade-banner-background.png";
            obj.title = "Chess";
            return obj;
        }
        return null;
    }

    initializeHTML(app) {
        super.initializeHTML(app);

        this.app.modules.respondTo("chat-manager").forEach(mod => {
            mod.respondTo('chat-manager').render(this_chess.app, this_chess);
        });
    }

    async initializeGame(game_id) {

        console.log('######################################################');
        console.log('######################################################');
        console.log('######################         #######################');
        console.log('######################  Saitoworlds###################');
        console.log('######################         #######################');
        console.log('######################################################');
        console.log('######################################################');

        if (this.browser_active == 1) {

            // enable chat
            //if (!this.app.browser.isMobileBrowser(navigator.userAgent)) {
            //  const chat = this.app.modules.returnModule("Chat");
            //  chat.addPopUpChat();
            //}

        }
    }

    returnGameOptionsHTML() {
        let html = `
      <div style="padding:40px;width:100vw;height:100vh;overflow-y:scroll;display:grid;grid-template-columns: 200px auto">
        <div style="top:0;left:0;margin-right: 20px;">
          <label for="color">Pick Your Race:</label>
          <select name="color">
            <option value="black" default>Black</option>
            <option value="white">White</option>
          </select>
        </div>
        <div>
          <div id="game-wizard-advanced-return-btn" class="game-wizard-advanced-return-btn button">accept</div>
        </div>
      </div>
    `;
        return html;
    }
}

module.exports = SaitoworldsGame;


