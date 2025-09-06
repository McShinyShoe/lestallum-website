// file: bot.js
const mineflayer = require("mineflayer");
const express = require("express");
const { once } = require("events");

const app = express();
const port = 3000;

app.use(express.json());

app.post("/mail", async (req, res) => {
  const { player, message } = req.body;

  let responded = false;
  function respond(data = null, message = '', code = 200, success = true, errors = null) {
    if(!responded) {
      responded = true;
      return res.status(code).json({ success: success, message: message, data: data, errors: errors, code: code });
    }
  };


  if (!player || !message) {
    return respond(null, "Missing 'player' or 'message' in JSON body", 400, false);
  }

  const bot = mineflayer.createBot({
    host: process.env.MINECRAFT_HOST,
    port: parseInt(process.env.MINECRAFT_PORT),
    username: process.env.MINECRAFT_USERNAME,
    version: process.env.MINECRAFT_VERSION,
    auth: "microsoft",
  });

  bot.once("spawn", async () => {
    try {
      bot.setQuickBarSlot(4);
      bot.activateItem();

      const [window] = await once(bot, "windowOpen");

      const containerSlot = 11;

      await bot.clickWindow(containerSlot, 1, 0);
    } catch (err) {
      console.error("Operation failed:", err);
    }
  });
  bot.on("message", (jsonMsg) => {
    const msg = jsonMsg.toString();
    if (msg === `[+] ${bot.username}`) {
      bot.emit("join");
    }
    if (msg.startsWith("♦")) {
      if(msg.includes("has been sent the following mail")) { 
        bot.removeAllListeners();
        bot.quit();
        return respond(null, "Mail has been sent succesfully");
      } else if (msg.includes("was never on this server")) {
        bot.removeAllListeners();
        bot.quit();
        return respond(null, "Player did not exist", 404, false);
      };
    }
  });
  bot.on("join", () => {
    bot.chat("/mail send " + player + " " + message);
  });
  bot.once("end", () => {
    return respond(null, "Bot disconnected before confirmation", 500, false);
  });
  bot.on("error", (err) => {
    console.error("Bot error:", err);
    return respond(null, "Error", 500, false, err.message);
  });
});

app.listen(port, () => {
  console.log(`Nodebot server running on port ${port}`);
});