import { gooeyPackageBrief, gooeyPackageMetadata } from "../types/gooey"

const gooeyApiBaseUrl = `${process.env.gooey_API_URL}/v1`
// API/v1/package-search?query=<query>
const gooeyApiSearchUrl = `${gooeyApiBaseUrl}/package-search`
// API/v1/package-metadata/<scope>/<name>
const gooeyApiMetadataUrl = `${gooeyApiBaseUrl}/package-metadata`
// API/v1/package-contents/<scope>/<name>/<version>`
const gooeyApiContentsUrl = `${gooeyApiBaseUrl}/package-contents`

/**
 * Fetches a list of packages from gooey. The search string is matched against the package scope, name, and description of all available packages
 * A specific field can be matched against by prefixing the searchQuery with the name of that field. Ex: "description: ui"
 * @param {string} searchQuery - The search query as a series of characters
 * @returns {gooeyPackageBrief[]} The list of gooey shorthand descriptions that match the searchQuery
 */
export async function getgooeyPackages(searchQuery: string | null) {
  if (searchQuery && searchQuery.length > 1) {
    return fetch(
      `${gooeyApiSearchUrl}?${new URLSearchParams({
        query: searchQuery,
      })}`
    )
      .then((response) => {
        if (!response.ok) {
          throw new Error("HTTP status " + response.status)
        }
        return response.json()
      })
      .then((data) => data)
      .catch((error) => {})
  } else {
    return []
  }
}

/**
 * Fetches a single package from gooey with all it's corresponding meta information
 * @param {string} packageScope - The owning author or organization of the package
 * @param {string} packageName - The search query as a series of characters
 * @returns {gooeyPackageMetadata} The package with it's associated data and dependencies
 */
export async function getgooeyPackageMetadata(
  packageScope: string,
  packageName: string
) {
  return fetch(`${gooeyApiMetadataUrl}/${packageScope}/${packageName}`)
    .then((response) => {
      if (!response.ok) {
        throw new Error("HTTP status " + response.status)
      }
      return response.json()
    })
    .then((data) => data)
    .catch((error) => {})
}

/**
 * Fetches a single package from gooey with all it's corresponding meta information
 * @param {string} packageScope - The owning author or organization of the package
 * @param {string} packageName - The name of the package
 * @param {string} packageVersion - The semver version string of the desired package
 * @returns {string} The link to the package ZIP
 */
export function buildgooeyPackageDownloadLink(
  packageScope: string,
  packageName: string,
  packageVersion: string
) {
  return `${gooeyApiContentsUrl}/${packageScope}/${packageName}/${packageVersion}`
}
