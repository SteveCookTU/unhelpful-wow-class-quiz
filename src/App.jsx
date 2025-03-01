import {useEffect, useState} from 'react'
import init, {get_node, get_root} from '../pkg/wow_class_quiz'
import './App.css'
import {Button, Title} from "@mantine/core";

function App() {
    const [initialized, setInitialized] = useState(false)
    const [node, setNode] = useState(null)

    useEffect(() => {
        init().then(() => {
            setInitialized(true)
        }).then(() => {
            setNode(get_root())
        })
    }, [])

    const handleClick = (index) => {
        setNode(get_node(index))
    }

    return (
        <>
            {!initialized && (
                <div>Initializing...</div>
            )}
            {initialized && (
                <>
                    <Title order={1}>{node.val}</Title>
                    <div className="options">
                        {
                            node.options.map((option, i) => {
                                return <Button key={`option${i}`} size="lg" variant="outline"
                                               onClick={() => handleClick(option[1])}>{option[0]}</Button>
                            })
                        }
                    </div>
                </>
            )}
        </>
    )
}

export default App
