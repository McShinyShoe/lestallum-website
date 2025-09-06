const mineflayer = require("mineflayer");
const express = require("express");

const app = express();
const port = 3000;

app.use(express.json());

app.post("/mail", async (req, res) => {
  const { player, message } = req.body;

  let responded = false;
  function respond(data = null, messageText = '', code = 200, success = true, errors = null) {
    if (!responded) {
      responded = true;
      return res.status(code).json({ success, message: messageText, data, errors, code });
    }
  }

  if (!player || !message) {
    return respond(null, "Missing 'player' or 'message' in JSON body", 400, false);
  }

  const { MINECRAFT_HOST, MINECRAFT_PORT, MINECRAFT_USERNAME, MINECRAFT_VERSION } = process.env;
  if (!MINECRAFT_HOST || !MINECRAFT_USERNAME) {
    return respond(null, "Server configuration missing (MINECRAFT_HOST or MINECRAFT_USERNAME)", 500, false);
  }

  const bot = mineflayer.createBot({
    host: MINECRAFT_HOST,
    port: MINECRAFT_PORT ? parseInt(MINECRAFT_PORT, 10) : undefined,
    username: MINECRAFT_USERNAME,
    version: MINECRAFT_VERSION,
    auth: "microsoft",
  });

  const onMessage = (jsonMsg) => {
    const msg = jsonMsg.toString();
    if (msg === `[+] ${bot.username}`) {
      bot.emit("join");
    }
    if (msg.startsWith("♦")) {
      if (msg.includes("has been sent the following mail")) {
        safeCleanup();
        try { bot.quit(); } catch (e) {}
        return respond(null, "Mail has been sent successfully");
      } else if (msg.includes("was never on this server")) {
        safeCleanup();
        try { bot.quit(); } catch (e) {}
        return respond(null, "Player did not exist", 404, false);
      }
    }
  };

  const onJoin = () => {
    try {
      bot.chat(`/mail send ${player} ${message}`);
    } catch (err) {
      console.error("Chat failed:", err);
      safeCleanup();
      try { bot.quit(); } catch (e) {}
      return respond(null, "Failed to send chat command", 500, false, err.message);
    }
  };

  const onSpawn = async () => {
    try {
      bot.setQuickBarSlot(4);
      bot.activateItem();

      const [window] = await oncePromise(bot, "windowOpen");
      const containerSlot = 11;
      await bot.clickWindow(containerSlot, 1, 0);
    } catch (err) {
      console.error("Operation failed during spawn:", err);
      safeCleanup();
      try { bot.quit(); } catch (e) {}
      return respond(null, "Operation failed during spawn", 500, false, err.message);
    }
  };

  const onError = (err) => {
    console.error("Bot error:", err && err.message ? err.message : err);
    safeCleanup();
    try { bot.quit(); } catch (e) {}
    return respond(null, "Bot error: " + (err && err.message ? err.message : String(err)), 500, false, err && err.stack ? err.stack : null);
  };

  const onEnd = () => {
    console.warn("Bot ended connection");
    safeCleanup();
    return respond(null, "Bot disconnected before confirmation", 500, false);
  };

  function safeCleanup() {
    try {
      bot.removeListener("message", onMessage);
      bot.removeListener("join", onJoin);
      bot.removeListener("spawn", onSpawn);
      bot.removeListener("end", onEnd);
    } catch (e) {}
  }

  bot.on("error", onError);
  bot.once("end", onEnd);

  bot.on("message", onMessage);
  bot.on("join", onJoin);
  bot.once("spawn", onSpawn);

  try {
    if (bot._client && bot._client.on) {
      bot._client.on("error", onError);
      bot._client.on("end", onEnd);
    }
  } catch (e) {}

  function oncePromise(emitter, event) {
    return new Promise((resolve, reject) => {
      const fn = (...args) => {
        cleanupOnce();
        resolve(args);
      };
      const onErr = (err) => {
        cleanupOnce();
        reject(err);
      };
      const cleanupOnce = () => {
        emitter.removeListener(event, fn);
        emitter.removeListener("error", onErr);
      };
      emitter.once(event, fn);
      emitter.once("error", onErr);
    });
  }
});

app.listen(port, () => {
  console.log(`Nodebot server running on port ${port}`);
});
