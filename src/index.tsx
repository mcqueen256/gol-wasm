import * as React from 'react'
import styles from './styles.module.css'

import { name } from "gol-wasm";

interface Props {
  text: string
}

export const ExampleComponent = ({ text }: Props) => {
  return <div className={styles.test}>Example Component ({name()}) {"=>"} {text}</div>
}
