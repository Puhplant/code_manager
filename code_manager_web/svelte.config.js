import { preprocessMeltUI, sequence } from '@melt-ui/pp'
 
const config = {
  preprocess: sequence([
    preprocessMeltUI()
  ])
}
 
export default config 