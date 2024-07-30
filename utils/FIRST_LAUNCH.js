import { $, file, fileURLToPath } from "bun";
import path from "node:path";
import open from "open";
import {
	intro,
	confirm,
	isCancel,
	cancel,
	note,
	password,
	select,
} from "@clack/prompts";
import pc from "picocolors";
import I18XS from "i18xs";
import { DATA, setData } from "~/utils/KEY";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
await $`mkdir -p /tmp/commitia`;
await $`touch /tmp/commitia/firstLaunch.json`;

const i18xs = new I18XS({
	supportedLocales: ["en", "es"],
	currentLocale: "en",
	fallbackLocale: "en",
	showMissingIdentifierMessage: false,
	missingIdentifierMessage: "Missing_Localization_Identifier",
	localesDir: path.join(__dirname, "..", "locales"),
	showLogs: false,
});

const userName = await $`whoami`.text();
const firstLaunchFile = file("/tmp/commitia/firstLaunch.json");
let jsonData = {};

async function firstLaunch() {
	console.clear();
	const askLanguagePreference = await select({
		message: "Select your language preference:",
		options: [
			{ value: "en", label: "English" },
			{ value: "es", label: "Español" },
		],
	});

	if (askLanguagePreference === "en") {
		i18xs.changeCurrentLocale("en");
		jsonData.lang = "en";
	} else if (askLanguagePreference === "es") {
		i18xs.changeCurrentLocale("es");
		jsonData.lang = "es";
	} else {
		console.error("Invalid language preference selected.");
	}

	if (isCancel(askLanguagePreference)) {
		cancel("operation cancelled");
		return process.exit(0);
	}

	console.clear();

	intro(
		`${pc.bgCyan(pc.black(`${i18xs.t("common.welcome_message")}, ${userName}`))}`,
	); // Welcome Message

	if (jsonData.lang === "en") {
		note(`  Welcome to the initial setup!  🚀
  In this initial configuration, you can choose how to manage your authentication token.
  Using your own token (recommended): guarantees greater security and control.
  Use the default token: A quick option to get started`);
	} else {
		note(`  ¡Bienvenido a la configuración inicial! 🚀
  En esta configuración inicial, puedes elegir cómo gestionar tu token de autenticación.

  Usar tu propio token (recomendado): garantiza mayor seguridad y control.
  Utilizar el token predeterminado: una opción rápida para comenzar.`);
	}

	const askContinue = await confirm({
		message: `${i18xs.t("common.ask_continue")}`,
		default: true,
	});

	if (isCancel(askContinue)) {
		cancel(`${i18xs.t("common.operation_cancelled")}`);
		return process.exit(0);
	}

	if (!askContinue) {
		console.log(
			pc.yellow(`
${i18xs.t("common.goodbye")}`),
		);
		return process.exit(0);
	}

	if (askContinue) {
		jsonData.exist = true;
	}

	const confirmUseOwnApi = await confirm({
		message: `${i18xs.t("common.ask_use_token")}`,
		default: true,
	}); // Groq API confirmation

	if (isCancel(confirmUseOwnApi)) {
		cancel(`${i18xs.t("common.operation_cancelled")}`);
		return process.exit(0);
	}

	if (confirmUseOwnApi) {
		if (jsonData.lang === "en") {
			note(`
  To obtain your Groq TOKEN, a new window will open in your default browser.
  If you already have a TOKEN, you can paste it here.
  `);
		} else {
			note(`
  Para obtener tu TOKEN de Groq, se abrirá una nueva ventana
  en tu navegador predeterminado. Si ya tienes un TOKEN,
  puedes pegarlo aquí.
  `);
		}

		await open("https://console.groq.com/keys");

		const tokenQuestion = await password({
			message: `${i18xs.t("common.paste_token")}`,
			validate: (value) => {
				if (!value) return `${i18xs.t("common.use_valid_token")}`;
			},
		});

		if (isCancel(tokenQuestion)) {
			cancel(`${i18xs.t("common.operation_cancelled")}`);
			return process.exit(0);
		}
		// Save token in temporal variable
		setData(tokenQuestion);
		jsonData.token = tokenQuestion;
	} else {
		jsonData.token = false;
	}

	// Ensure DATA is valid if token is provided
	if (jsonData.token && (!DATA || DATA.trim() === "")) {
		console.error(pc.red(`${i18xs.t("common.invalid_token")}`));
		return process.exit(1);
	}

	const writer = firstLaunchFile.writer();
	writer.write(JSON.stringify(jsonData, null, 2));
	await writer.end();
}

export { firstLaunch, i18xs };
