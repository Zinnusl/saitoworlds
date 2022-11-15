const GameTemplate = require('../../lib/templates/gametemplate');

class SaitoworldsGame extends GameTemplate {

    constructor(app) {
        super(app);

        this.name = "Saitoworlds";
        this.description = "Saitoworlds is a infinite player 4x game played on an infinite hex grid."
        this.publickey = app.wallet.returnPublicKey();

        this.useHUD = 0;
        this.useClock = 0;

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
        console.log("respondTo: " + type);

        if (type == "arcade-carousel") {
            const obj = {};
            obj.background = "/chess/img/arcade/arcade-banner-background.png";
            obj.title = "Saitoworlds";
            return obj;
        }

        if (super.respondTo(type) != null) {
            return super.respondTo(type);
        }

        return null;
    }

    testerino(str) {
        let newtx = this.app.wallet.createUnsignedTransactionWithDefaultFee();  // if no recipient, send to ourselves!
        newtx.msg.module  = "Saitoworlds";
        newtx.msg.title   = "SerialisedGameState";
        newtx.msg.serde = str;
        newtx = this.app.wallet.signTransaction(newtx);
        this.app.network.propagateTransaction(newtx);
        return 22;
    }

    initializeHTML(app) {
        super.initializeHTML(app);

        this.app.modules.respondTo("chat-manager").forEach(mod => {
            mod.respondTo('chat-manager').render(this_chess.app, this_chess);
        });
    }

    initializeGame(_game_id) {

        console.log('######################################################');
        console.log('######################################################');
        console.log('######################               #################');
        console.log('######################  Saitoworlds  #################');
        console.log('######################               #################');
        console.log('######################################################');
        console.log('######################################################');

        // Koennte sein, dass this nicht mehr das ist, was es sein sollte
        window.saitoworlds_module = this;

        if (this.browser_active == 1) {

            // enable chat
            //if (!this.app.browser.isMobileBrowser(navigator.userAgent)) {
            //  const chat = this.app.modules.returnModule("Chat");
            //  chat.addPopUpChat();
            //}

        }
    }

    async onConfirmation(blk, tx, confnum, app) {
        if (this.browser_active == 0) { return; }
        url = new URL(window.location.href);
        if (url.searchParams.get('module') != null) { return; }

        if (this.wasm_onConfirmationCallback) {
            this.wasm_onConfirmationCallback(tx.msg.serde);
        }
    }

    onPeerHandshakeComplete(app, peer) {
        console.log("js onPeerHandshakeComplete 1");
        if (this.browser_active == 0) { return; }
        url = new URL(window.location.href);
        if (url.searchParams.get('module') != null) { return; }
        console.log("js onPeerHandshakeComplete 2");

        // txs are saito\transaction.js
        // this.app.storage.loadTransactions("Saitoworlds", 500000, (txs) => {

        //     // txs = txs.filter(tx => tx.msg.serde);
        //     console.log("js loadTransactions txs: ", JSON.stringify(txs));
        //     txs = txs.map(tx => {
        //         return tx.msg.serde || "";
        //     });

        //     if (this.wasm_onLoadTransactionsCallback) {
        //         console.log("js wasm_onLoadTransactionsCallback");
        //         this.wasm_onLoadTransactionsCallback(txs);
        //     }
        // });

        let sql = "";
        let params = {};

        if (type === "all") {
            sql = "SELECT * FROM txs WHERE publickey = $publickey ORDER BY id DESC LIMIT $num";
            params = { $publickey : publickey , $num : num};
        } else {
            sql = "SELECT * FROM txs WHERE publickey = $publickey AND type = $type ORDER BY id DESC LIMIT $num";
            params = { $publickey : publickey , $type : type , $num : num};
        }

        let rows = await this.app.storage.queryDatabase(sql, params, "archive");
        let txs = [];

        if (rows != undefined) {
            if (rows.length > 0) {
                txs = rows.map(row => row.tx);
            }
        }
        return txs;

    }

    returnGameOptionsHTML() {
        const html = `
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


