const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

const VerifierModule = buildModule("VerifierModule", (m) => {
  const verifier = m.contract("Verifier");
  return { verifier };
});

module.exports = VerifierModule;
