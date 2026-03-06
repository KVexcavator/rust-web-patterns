import init, { rust_generate_button_text } from
'../rust-interface/pkg/rust_interface.js';
import "./App.css";
import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import { ToDoItems } from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import { CreateToDoItem } from './components/CreateItemForm';

const App = () => {
    const [data, setData] = useState(null);
    const [error, setError] = useState(null);
    const [wasmReady, setWasmReady] = useState<boolean>(false);
    const [
        RustGenerateButtonText,
        setRustGenerateButtonText
    ] = useState<(input: string) => string>(null);

    React.useEffect(() => {
        init().then(() => {
            setRustGenerateButtonText(() => rust_generate_button_text);
            setWasmReady(true);
        }).catch(e => console.error(
            "Error initializing WASM: ", e
        ));
    }, []);


    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();

            if (response.error) {
                setError(response.error);
            } else {
                setData(response.data);
            }
        };
        
        if (wasmReady) {
            fetchData();
        }
    }, [wasmReady]);

    function reRenderItems(response: any) {
        if (response.error) {
            alert(JSON.stringify(response));
            return;
        }
        else if (response.data) {
            setData(response.data);
            setError(null);
        }
        else {
            setError("Unknown error");
        }
    }

    if (error) {
        return <div style={{ color: 'red' }}>Error: {error}</div>
    }
    else if (!data) {
        return <div>Loading...</div>
    }
    else {
        return (
            <div className="App">
            <div className="mainContainer">
                <div className="header">
                    <p>complete tasks: {data.done.length}</p>
                    <p>pending tasks: {data.pending.length}</p>
                </div>
                <h1>Pending Items</h1>
                <div>
                    {data.pending.map((item, index) => (
                        <>  
                            <ToDoItem 
                                key={item.title + item.status}
                                title={item.title}
                                id={item.id}
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }
                                passBackResponse={reRenderItems}
                            />
                        </>
                    ))}
                </div>
                <h1>Done Items</h1>
                <div>
                    {data.done.map((item, index) => (
                        <>  
                            <ToDoItem 
                                key={item.title + item.status}
                                title={item.title}
                                id={item.id}
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }
                                passBackResponse={reRenderItems}
                            />
                        </>
                    ))}
                </div>
                <CreateToDoItem passBackResponse={reRenderItems} />
            </div>
            </div>
        );
    }
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);