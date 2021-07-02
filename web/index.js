import React from "react";
import ReactDOM from "react-dom";
import styled, { createGlobalStyle } from "styled-components";
import { Row, Col } from "./components/FlexboxGrid.jsx";
import { solve } from "../Cargo.toml";

const GlobalStyle = createGlobalStyle`
  body {
    background: #ddebf7;
  }
  input[type=number]::-webkit-inner-spin-button,
  input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
`;

const Page = styled(Col)`
  max-width: 50rem;
  margin: 0 auto;
`;

const Numbox = styled.input`
  border: 2px solid #309ee4;
  text-align: center;
  width: 3rem;
  padding: 0.5rem 1rem;
  margin-top: 2rem;
  font-size: xx-large;
  background: #f7f9fd;
`;

function NumberBox(props) {
  return <Numbox type="number" {...props} />;
}

const Button = styled.button`
  margin-top: 2rem;
  width: 9rem;
  padding: 0.5rem 1rem;
  font-size: 1.2rem;
  cursor: pointer;
  color: white;
  border-radius: 6px;
  background-color: white;
  background-image: linear-gradient(rgb(7, 136, 222), rgb(17, 106, 184));
  &:hover {
    background-image: linear-gradient(rgb(6, 121, 197), rgb(15, 93, 161));
  }
`;

const Answer = styled.textarea`
  border: 2px solid #309ee4;
  height: 22rem;
  margin-top: 2rem;
  font-size: xx-large;
  background: #f7f9fd;
`;

class Calculator extends React.Component {
  state = {
    numbers: [4, 4, 4, 4],
    solution: ""
  };

  next = () => {
    let number = () => Math.ceil(Math.random() * 13);
    let numbers = [number(), number(), number(), number()];
    while (solve(numbers) == "") {
      numbers = [number(), number(), number(), number()];
    }
    this.setState({ numbers });
  };

  compute = () => {
    const { numbers } = this.state;
    const result = solve(numbers);
    const solution =
      result == "" ? "No solution found." : result.split(";").join("\n");
    this.setState({ solution });
  };

  setNumber = index => {
    return e => {
      let { numbers } = this.state;
      numbers[index] = e.target.value;
      this.setState({ numbers });
    };
  };

  render() {
    const { numbers } = this.state;
    return (
      <Page>
        <GlobalStyle />
        <h1>24 Point Calculator</h1>
        <Col>
          <Row space="around" nowrap>
            <NumberBox value={numbers[0]} onChange={this.setNumber(0)} />
            <NumberBox value={numbers[1]} onChange={this.setNumber(1)} />
            <NumberBox value={numbers[2]} onChange={this.setNumber(2)} />
            <NumberBox value={numbers[3]} onChange={this.setNumber(3)} />
          </Row>
          <Row space="around">
            <Button onClick={this.next}>Next Group</Button>
            <Button onClick={this.compute}>Solve</Button>
          </Row>
          <Answer readOnly value={this.state.solution} />
        </Col>
      </Page>
    );
  }
}

ReactDOM.render(<Calculator />, document.getElementById("app"));
