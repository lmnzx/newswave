import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { useState } from 'react'

export default function PublishNewsLetter() {

  const [value, setValue] = useState('');

  return (
    <>
      <ReactQuill theme="snow" value={value} onChange={setValue} />
      <button onClick={() => console.log(value)}>submit</button>
    </>
  )
}
