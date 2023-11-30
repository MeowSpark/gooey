import { rbxpmPackageBrief, rbxpmPackageMetadata } from "../types/rbxpm"

const rbxpmApiBaseUrl = `${process.env.rbxpm_API_URL}/v1`
// API/v1/package-search?query=<query>
const rbxpmApiSearchUrl = `${rbxpmApiBaseUrl}/package-search`
// API/v1/package-metadata/<scope>/<name>
const rbxpmApiMetadataUrl = `${rbxpmApiBaseUrl}/package-metadata`
// API/v1/package-contents/<scope>/<name>/<version>`
const rbxpmApiContentsUrl = `${rbxpmApiBaseUrl}/package-contents`

/**
 * Fetches a list of packages from rbxpm. The search string is matched against the package scope, name, and description of all available packages
 * A specific field can be matched against by prefixing the searchQuery with the name of that field. Ex: "description: ui"
 * @param {string} searchQuery - The search query as a series of characters
 * @returns {rbxpmPackageBrief[]} The list of rbxpm shorthand descriptions that match the searchQuery
 */
export async function getrbxpmPackages(searchQuery: string | null) {
  if (searchQuery && searchQuery.length > 1) {
    return fetch(
      `${rbxpmApiSearchUrl}?${new URLSearchParams({
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
 * Fetches a single package from rbxpm with all it's corresponding meta information
 * @param {string} packageScope - The owning author or organization of the package
 * @param {string} packageName - The search query as a series of characters
 * @returns {rbxpmPackageMetadata} The package with it's associated data and dependencies
 */
export async function getrbxpmPackageMetadata(
  packageScope: string,
  packageName: string
) {
  return fetch(`${rbxpmApiMetadataUrl}/${packageScope}/${packageName}`)
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
 * Fetches a single package from rbxpm with all it's corresponding meta information
 * @param {string} packageScope - The owning author or organization of the package
 * @param {string} packageName - The name of the package
 * @param {string} packageVersion - The semver version string of the desired package
 * @returns {string} The link to the package ZIP
 */
export function buildrbxpmPackageDownloadLink(
  packageScope: string,
  packageName: string,
  packageVersion: string
) {
  return `${rbxpmApiContentsUrl}/${packageScope}/${packageName}/${packageVersion}`
}
