const ModTemplate = require('../../lib/templates/gametemplate');

class SaitoworldsGame extends ModTemplate {

    constructor(app) {
        super(app);

        this.name = "Saitoworlds";
        this.description = "Saitoworlds is an art experiment similar to r/place's yearly art event."
        this.publickey = app.wallet.returnPublicKey();

        this.useHUD = 0;
        this.useClock = 0;

        this.minPlayers = 1;
        this.maxPlayers = 9;
        this.type       = "Art";
        this.categories  = "Art";

        const is_browser = typeof window != "undefined";
        if (is_browser) {
            window.saitoworlds_module = this;
            this.browser_active = true;
        }
        else {
            global.saitoworlds_module = this;
            this.browser_active = false;
        }

        return this;
    }
    
    getTransactions(app) {
        return new Promise((resolve, _reject) => {
            resolve(app.blockchain.index.blocks.map(block => {
                return block.transactions.map(tx => {
                    return tx.msg.serde || "";
                });
            }).flat().filter(tx => tx != ""));


                // const txs_filtered = txs.filter(tx => tx.msg.module === "Saitoworlds" && tx.msg.title === "SerialisedGameState");
            // console.log("getTransactions in expressApp 1: " + JSON.stringify(app.blockchain.index.blocks));
            // if (txs) {
            //     txs = txs.map(tx => {
            //         return tx.msg.serde || "";
            //     });
            //     console.log("getTransactions in expressApp 2: " + txs.length);
            //     resolve(txs);
            // }
            // else {
            //     resolve([]);
            // }
        });
    }

    webServer(app, expressapp, express) {
        super.webServer(app, expressapp, express);

        const self = app.modules.returnModule("Saitoworlds");

        // TODO do this with the socketio connection?
        expressapp.get('/saitoworlds/transactions/', async function (_req, res) {
            res.setHeader('Content-type', 'text/json');
            res.charset = 'UTF-8';
            const txs = await self.getTransactions(app);
            res.write(JSON.stringify(txs));
            res.end();
            return;
        });

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
        // super.initializeHTML(app);

        // this.app.modules.respondTo("chat-manager").forEach(mod => {
        //     mod.respondTo('chat-manager').render(this_chess.app, this_chess);
        // });
    }

    async onConfirmation(blk, tx, confnum, app) {
        if (this.browser_active == 0) { return; }
        url = new URL(window.location.href);
        if (url.searchParams.get('module') != null) { return; }

        if (this.wasm_onConfirmationCallback) {
            this.wasm_onConfirmationCallback(tx.msg.serde);
        }
    }
    
    httpGetAsync(theUrl, callback) {
        var xmlHttp = new XMLHttpRequest();
        xmlHttp.onreadystatechange = function() {
            if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
                callback(xmlHttp.responseText);
        }
        xmlHttp.open("GET", theUrl, true); // true for asynchronous
        xmlHttp.send(null);
    }

    onPeerHandshakeComplete(app, peer) {
        if (!this.browser_active) { return; }

        this.httpGetAsync("http://localhost:12101/saitoworlds/transactions", (response) => {
            const txs = JSON.parse(response);
            if (this.wasm_onLoadTransactionsCallback) {
                this.wasm_onLoadTransactionsCallback(txs);
            }
        });
    }

}

module.exports = SaitoworldsGame;


