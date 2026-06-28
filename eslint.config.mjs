// @ts-check
import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
	allowIndentationTabs: true,
	tabLength: 4,
	"no-tabs": 0,
	ident: ["tab"]
});
