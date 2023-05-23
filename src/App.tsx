import { Routes, Route } from "react-router-dom";

import "./App.css";
import { Equivalencies } from "./equivalencies/equivalencies";
import { Home } from "./home/home";
import { ConditionalRendering } from "./equivalencies/conditional_rendering/conditional_rendering";
import { Logging } from "./equivalencies/logging/log";
import { OnClick } from "./equivalencies/onclick/onclick";

function App() {
  return (
    <>
      <Routes>
        <Route index path="/" element={<Home />}></Route>
        <Route path="equivalencies" element={<Equivalencies />}>
          <Route
            index
            path="conditional-rendering"
            element={<ConditionalRendering variant="primary" />}
          />
          <Route path="logging" element={<Logging />} />
          <Route path="on-click" element={<OnClick />} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
