import { getOsExploreInfo, initCustomTraceSubscriber } from "../index.js"

init()

async function init() {
  initCustomTraceSubscriber("./logs")
  setTimeout(() => {
    const info = getOsExploreInfo()
    console.log(info)
  }, 1000)
}
