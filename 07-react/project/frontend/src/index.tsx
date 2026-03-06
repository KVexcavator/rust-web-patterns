// File: frontend/src/index.tsx
import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { CreateToDoItem } from './components/CreateItemForm';

const App = () => {
    const [data, setData] = useState(null);
    const [error, setError] = useState(null); // State to store error messages

    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();

            if (response.error) {
                setError(response.error);
            } else {
                setData(response.data);
            }
        };
        
        fetchData();
    }, []);

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

    return (
        <div>
            <h4>Title</h4>
            {error ? (
                <div style={{ color: 'red' }}>Error: {error}</div>
            ) : data ? (
                <div>Data loaded: {JSON.stringify(data)}</div>
            ) : (
                <div>Loading...</div>
            )}
            <CreateToDoItem passBackResponse={reRenderItems} />
        </div>
    );
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);