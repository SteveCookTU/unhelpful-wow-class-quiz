import {useEffect, useState} from 'react'
import init, {get_node, get_root} from '../pkg/wow_class_quiz'
import './App.css'
import {Anchor, Button, Container, Title} from "@mantine/core";
import classes from './FooterSimple.module.css';

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
                    <div className="content">
                        <Title order={1} ta="center" className="title">{node.val}</Title>
                        <div className="options">
                            {
                                node.options.map((option, i) => {
                                    return <Button key={`option${i}`} size="lg" variant="outline"
                                                   onClick={() => handleClick(option[1])}>{option[0]}</Button>
                                })
                            }
                        </div>
                        <div className={classes.footer}>
                            <Container className={classes.inner}>
                                <div>Inspired by <a href="https://www.reddit.com/r/wow/comments/kymbvz/as_promised_a_new_and_improved_version_of_my/" target="_blank">this Reddit post</a> from u/delicatedead</div>
                            </Container>
                        </div>
                    </div>

                </>
            )}
        </>
    )
}

export default App
